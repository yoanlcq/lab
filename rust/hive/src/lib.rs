// Ce que je veux pouvoir faire:
// - Ajouter pendant l'itération
//   Ca ajoute un élément sans réallocation de storage existant (pour pas casser les pointeurs live)
//   On a une API is_pending_add() sur n'importe quel item: ça renvoie true si l'item n'est pas dans le 1er buffer.
//   Un forward iterator peut donc choisir de break early dès qu'il tombe sur un pending add.
// - Supprimer pendant l'itération
//   Ca "mark for delete". On a une API is_pending_removal() sur n'importe quel item.
//   Un iterator peut quand même choisir de processer ces éléments.
// - Compaction
//   Ca applique les pending removals puis les pending adds.
// - Sort
//   On peut set un prédicat de comparaison pour trier.
//   Plusieurs choix:
//   - Prédicat user-defined
//   - Ordre "chronologique" (les derniers ajoutés sont à la fin; les removals préservent l'ordre)
//   - Ordre complètement indéfini (permet de faire des swap_remove)
// - Weak refs ("handles")
//   On peut obtenir une weak ref à partir de n'importe quel item
//   La weak ref utilise des pointeurs afin d'être sympa dans une watch windows de debugger
// - Toute Weak ref peut être upgradée en strong ref tant que l'item est encore présent dans le container
// - Le container a une strong ref sur chaque item par défaut; cette strong ref peut être "volée" ou "consommée"
// - Une "ComponentRef" c'est { WeakOrStrongRef<Entity>, WeakOrStrongRef<TheComponentData> }
//   Toute permutation de weak/strong est possible dans les deux membres, ce qui permet toute gestion alambiquée que l'on peut imaginer
// - Un component dérivé référence sa "base" via StrongRef<BaseComponent>.
//   Lorsque le refcount d'une Entity atteint 0, on clear sa liste de StrongRef<Component>.
//   Vu que chaque component dérivé a une StrongRef sur sa base, les components dérivés vont être drop en premier, puis ensuite les bases.
// - Les components ne doivent jamais avoir de StrongRef sur leur propre entité
//   Chaque entité a une liste de StrongRef de ses components, donc cela créerait un cycle.
// - On pourrait imaginer un cas où une Entity a une WeakRef sur un de ses components au lieu d'une StrongRef.
//   Le component pourrait alors avoir une StrongRef sur son Entity, sans créer de cycle.
//   Cela fait que l'Entity ne peut pas être supprimée à moins que quelqu'un ne supprime d'abord le component.
//
// Notion de contexte:
// - process_context (the root of all. there is always only one)
//   - gpu_api_context ?
//     - gpu_device_context ?
//   - audio_api_context ?
//     - audio_device_context ?
//   - editor_context ?
//     - editor_user_context ?
//   - game_context (in a shipping game OR in a sandbox within the editor. Has a set of "viewports" (each either a window or sub-window). Running multiple game instances is useful for testing network functionality while sharing assets in memory)
//     - world_context (in one game, you can have N worlds running in parallel (they each have their time_since_start, simulation, etc, but may not necessarily contribute audio/video output))
//     - local_player_context (each game_context tracks its own connected local players)
//     - remote_player_context
//
// Gestion de contexte :
// - Pas de globals. Cela permet des tests unitaires isolés/sandboxés au sein du process.
//   Surtout par exemple des tests du système d'ECS, d'alloc mémoire, défragmentation, etc
// - Les contextes sont ref-countés et passés aux fonctions.
//   Cependant, il est facile de récupérer le contexte d'une certaine entité (cela se fait en récupérant un component spécial qu'elle est censée avoir selon la situation).
//   Donc si c'est bien fait, on ne devrait pas être inondés de contextes ajoutés en paramètres partout, car presque n'importe quel objet permet de les récupérer.
// - Les contextes sont hiérarchisés; il est facile de choper un contexte ancêtre d'un autre contexte.
// - Les services providers sont obtenables depuis n'importe quel contexte.
//   exemple: un world_context peut avoir son propre log service provider. Ca va prendre le dessus sur le log service provider des autres contextes (mais lui-même peut dire "j'appelle le parent").
//   Donc quand tu fais log!(x, "hello"), ça chope le contexte le plus spécialisé obtenable via x, puis ça cherche le service provider le plus proche.
//   Si besoin de garder le service provider en cache, possible de faire un truc genre LogServiceProvider::find(x).

pub mod hive {
    use std::rc::Rc;
    use std::cell::{Cell, RefCell};

    mod imp {
        use std::{cell::{Cell, RefCell, UnsafeCell}, mem::MaybeUninit, rc::{Rc, Weak}};
        use std::pin::Pin;
        
        #[derive(Debug)]
        pub struct PinArena<T> {
            memory: Box<[UnsafeCell<MaybeUninit<T>>]>,
            len: Cell<usize>,
        }

        impl <T> PinArena<T> {
            pub fn with_capacity(cap: usize) -> Self {
                assert_ne!(cap, 0);
                Self {
                    memory: (0..cap).map(|_| UnsafeCell::new(MaybeUninit::uninit())).collect::<Vec<_>>().into_boxed_slice(),
                    len: Cell::new(0),
                }
            }
            pub fn capacity(&self) -> usize {
                self.memory.len()
            }
            pub fn is_full(&self) -> bool {
                self.len.get() >= self.capacity()
            }
            pub fn try_push(&self, val: T) -> Option<Pin<&mut T>> {
                if self.is_full() {
                    None
                } else {
                    unsafe { Some(self.push_unchecked(val)) }
                }
            }
            pub unsafe fn push_unchecked(&self, val: T) -> Pin<&mut T> {
                let i = self.len.get();
                self.len.set(i + 1);
                Pin::new_unchecked((*self.memory[i].get()).write(val))
            }
        }

        pub struct PinArenaListNode<T> {
            pin_arena: PinArena<T>,
            next: RefCell<Option<Rc<PinArenaListNode<T>>>>,
            prev: RefCell<Option<Weak<PinArenaListNode<T>>>>,
        }

        impl<T> PinArenaListNode<T> {
            pub fn with_capacity(cap: usize) -> Self {
                Self {
                    pin_arena: PinArena::with_capacity(cap),
                    next: RefCell::new(None),
                    prev: RefCell::new(None),
                }
            }
        }

        pub struct PinArenaList<T> {
            head: Rc<PinArenaListNode<T>>,
            tail: RefCell<Rc<PinArenaListNode<T>>>,
        }

        impl<T> PinArenaList<T> {
            pub fn with_capacity(cap: usize) -> Self {
                assert_ne!(cap, 0);
                let tail = Rc::new(PinArenaListNode::with_capacity(cap));
                Self {
                    head: Rc::clone(&tail),
                    tail: RefCell::new(tail),
                }
            }

            pub fn push(&self, val: T) {
                if self.tail.borrow().pin_arena.is_full() {
                    let new_tail = Rc::new(PinArenaListNode {
                        pin_arena: PinArena::with_capacity(64),
                        prev: RefCell::new(Some(Rc::downgrade(&self.tail.borrow()))),
                        next: RefCell::new(None),
                    });
                    let mut tail = self.tail.borrow_mut();
                    *tail.next.borrow_mut() = Some(Rc::clone(&new_tail));
                    *tail = new_tail;
                }

                unsafe { self.tail.borrow().pin_arena.push_unchecked(val); }
            }
        }

        pub struct PinnedHandlePayload<T> {
            item_ptr: *const T,
            generation: std::num::Wrapping<usize>,
        }

        pub struct Handle<T> {
            payload: *const PinnedHandlePayload<T>,
            generation: std::num::Wrapping<usize>,
        }
    }

    #[derive(Debug, Clone)]
    pub struct Hive<T> {
        items: Vec<T>,
        nodes: RefCell<Vec<Rc<imp::PinArena<T>>>>,
        lock_counter: Cell<usize>,
    }

    impl<T> Hive<T> {
        pub fn add(&self, item: T) {
            /*
            if !self.is_locked() {
                self.items.push(item);
            } else {
                let mut vv = self.nodes.borrow_mut();
                if vv.is_empty() || vv.last().unwrap().is_full() {
                    vv.push(Rc::new(imp::PinArena::with_capacity(64)));
                }
                unsafe { vv.last_mut().unwrap().push_unchecked(item); }
            }
             */
        }
        pub fn is_locked(&self) -> bool {
            self.lock_counter.get() > 0
        }
        pub fn increment_lock_counter(&self) {
            self.lock_counter.set(self.lock_counter.get() + 1);
        }
        pub fn decrement_lock_counter(&self) {
            self.lock_counter.set(self.lock_counter.get() - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}