use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Carte {
    classe: Classe,
    nom: &'static str,
    illustration: &'static str,
    effet: &'static str,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Classe {
    Pouvoir, Entite
}

impl Default for Carte {
    fn default() -> Carte {
        Carte {
            classe: Classe::Pouvoir,
            nom: "Sans nom",
            illustration: "_.png",
            effet: "Sans effet",
        }
    }
}

macro_rules! liste_des_cartes {
    ($({$Classe:ident $Nom:ident $NOM:ident $nom:expr, $illustration:expr, $effet:expr})+) => {
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum Reference {
            $($Nom,)+
        }
        $(
            pub static $NOM: Carte = Carte {
                classe: Classe::$Classe,
                nom: $nom,
                illustration: $illustration,
                effet: $effet,
            };
        )+
        pub fn cartes() -> HashMap<Reference, &'static Carte> {
            let mut h = HashMap::new();
            $(h.insert(Reference::$Nom, &$NOM);)+
            h
        }
    }
}

// But: Etre imprédictible et surprendre. Retourner/Chambouler la situation brutalement
// - Une dose de destruction pour s'opposer au joueur de la Nuit
// - TODO espoirs et rêves
pub mod aube {
    use super::*;
    liste_des_cartes!{
        { Pouvoir Ultimatum ULTIMATUM "Ultimatum", "Ultimatum.png",
"Désignez une carte de la main d'un adversaire.
A la fin du prochain tour adverse, si la carte désignée n'a pas été jouée, vous et votre partenaire pourrez piocher autant de cartes que vous le souhaitez à la place de votre pioche normale au début de votre prochain tour." }
        { Pouvoir Initialisation            PN "Sans nom", "_.png",
"Piochez deux cartes. Si cette carte est la première à avoir été jouée de la partie, piochez autant de cartes que vous le souhaitez à la place." }
        { Pouvoir Commencement              PK "Sans nom", "_.png",
"Révélez les 5 cartes du dessus d'un deck. Si cette carte est la première à avoir été jouée de la partie, révélez l'entièreté d'un deck à la place, excluant la carte nommée pour votre condition de victoire (demandez à l'adversaire concerné de révéler les cartes lui-même afin de ne pas révéler la carte nommée)." }
        { Pouvoir Depart                    PXX"Sans nom", "_.png",
"Vos adversaires défaussent chacun 1 carte. Si cette carte est la première à avoir été jouée de la partie, vos adversaires défaussent leur main à la place." }
        { Pouvoir Annonce                   PR "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Information               PP "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Apprentissage             PD "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Analyse                   PB "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Observation               PF "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Intersection              PM "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Vision                    PU "Sans nom", "_.png",
"Regardez les cartes du deck ciblé sans le mélanger." }
        { Pouvoir Planification             PO "Sans nom", "_.png",
"Regardez jusqu'à 10 cartes du dessus du deck ciblé et replacez-les dans l'ordre de votre choix." }
        { Pouvoir Prevoyance                PW "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Prediction                PA "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Speculation               PC "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Premonition               PV "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Divination                PE "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Inference                 PH "Sans nom", "_.png", "Sans effet" }
        { Pouvoir RechercheEtDeveloppement  PS "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Optimisation              PL "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Devoir                    PI "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Deduction                 PG "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Decision                  PQ "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Consequence               PY "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Destinee                  PJ "Sans nom", "_.png",
"Jusqu'au début de votre prochain tour, quand un adversaire veut jouer une carte de sa main: vous pouvez regarder sa main et choisir la carte qui sera jouée à la place." }
        { Entite  LaConstellation           ET "Sans nom", "_.png",
"Les joueurs jouent avec leur main révélée." }
        { Entite  LEtoileDuNord             EA "Sans nom", "_.png",
"* Les étoiles ne m'ont pas redonné mon héritage, mais je peux compter sur elles pour me guider vers ma destinée." }
        { Entite  LaPierreDeCristaux        EB "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaSagesse                 EC "Sans nom", "_.png", "Sans effet" } 
        { Entite  LePhilosophe              ED "Sans nom", "_.png", "Sans effet" } 
        { Entite  ConceptBLeBus             EE "Sans nom", "_.png", "Sans effet" } 
        { Entite  LePredicteurDeBranches    EF "Sans nom", "_.png",
"Lorsque cette carte arrive en jeu, ciblez un joueur.

Au début de chaque tour de ce joueur, juste avant sa phase de pioche" }
        { Entite  LeProcesseur              EG "Sans nom", "_.png", "Sans effet" } 
        { Entite  LIntelligenceArtificielle EH "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaCalculatrice            EI "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeBonEntendeur            EJ "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeFuturiste               EK "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeVisionnaire             EL "Sans nom", "_.png", "Sans effet" } 
        { Entite  LInnovatrice              EM "Sans nom", "_.png",
"Lorsque cette carte arrive sur le terrain, ciblez un joueur.
Vos conditions de victoire sont échangées tant que \"this\" se trouve sur le terrain." } 
        { Entite  LaBravoure                EN "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaFortune                 EO "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeChercheur               EP "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaTraductrice             EQ "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaFoi                     ER "Sans nom", "_.png", "Sans effet" } 
        { Entite  LAudace                   ES "Sans nom", "_.png", "Sans effet" } 
    }
}

pub mod jour {
    use super::*;
    liste_des_cartes!{
        { Pouvoir Clameur                PA "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Gloire                 PB "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Succes                 PC "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Union                  PD "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Somme                  PE "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Synergie               PF "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Superposition          PG "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Echange                PY "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Partage                PH "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Amitie                 PI "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Amour                  PJ "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Naissance              PK "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Presence               PL "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Connexion              PM "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Parallelisme           PN "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Epanouissement         PO "Sans nom", "_.png", "Sans effet" }
        { Pouvoir ConscienceCollective   PP "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Justice                PQ "Sans nom", "_.png",
"Le joueur de la Nuit envoie tous ses permanents dans l'Aether." }
        { Pouvoir Equilibre              PR "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Optimisme              PS "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Efficacite             PT "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Purete                 PU "Sans nom", "_.png",
"Sans effet" }
        { Pouvoir Innocence              PV "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Benediction            PW "Sans nom", "_.png", "Sans effet" }
        { Entite  LArtiste               EA "Sans nom", "_.png", "Sans effet" } 
        { Entite  LArtisan               EB "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeChevalier            EC "Sans nom", "_.png", "Sans effet" } 
        { Entite  LArchitecte            ED "Sans nom", "_.png", "Sans effet" } 
        { Entite  ConceptCLeConstructeur EE "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaDanseuse             EF "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaChanteuse            EG "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeJoueur               EH "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaTherapeute           EI "Sans nom", "_.png", "Sans effet" } 
        { Entite  LAventuriere           EJ "Sans nom", "_.png", "Sans effet" } 
        { Entite  LExploratrice          EK "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaJoie                 EL "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeFou                  EM "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaLiberte              EN "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeGentilhomme          EO "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaMere                 EP "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeSoleil               EQ "Sans nom", "_.png", "Sans effet" } 
    }
}

// But: Faire meuler et remplir les Aethers (i.e avoir vu la totalité du passé)
// - Faire revenir de l'Aether
// - Piocher ou faire piocher
// - Jouer ou faire jouer
// - Faire défausser
// - Faire meuler
// - Jouer un tour de plus
pub mod crepuscule {
    use super::*;
    liste_des_cartes!{
        { Pouvoir Protection            PA "Sans nom", "_.png", 
"Le joueur ciblé défausse sa main et pioche autant de cartes." }
        { Pouvoir Preservation          PU "Sans nom", "_.png", 
"L'alliance adverse envoie un total de 10 cartes du dessus de ses decks dans l'Aether, puis chaque adversaire récupère une carte de son choix depuis son Aether vers sa main." }
        { Pouvoir Conservation          PQ "Sans nom", "_.png",
"Vous jouez un tour de plus à la fin de ce tour." }
        { Pouvoir Gestion               PV "Sans nom", "_.png",
"Déclarez un nombre positif N (N peut valoir 0).
Mélangez N cartes de l'Aether de votre alliance dans leurs decks respectifs, puis envoyez un total de N+2 cartes du dessus de vos decks vers l'Aether." }

        { Pouvoir Rigueur               PB "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Discipline            PC "Sans nom", "_.png",
"Le joueur ciblé défausse (ou bien pioche) des cartes jusqu'à en avoir autant qu'un autre joueur ciblé." }
        { Pouvoir Maitrise              PD "Sans nom", "_.png",
"Vous pouvez jouer jusqu'à deux autres cartes ce tour." }
        { Pouvoir Patience              PE "Sans nom", "_.png",
"Mettez de côté, face verso, jusqu'à 2 cartes du dessus du deck d'un joueur ciblé.
Lors du prochain tour de ce joueur, avant la phase de pioche, ces cartes seront révélées et jouées." }
        { Pouvoir Fermete               PF "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Lecon                 PZ "Sans nom", "_.png", "Sans effet" }

        { Pouvoir Resistance            PG "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Regret                PI "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Revanche              PJ "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Riposte               PK "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Replique              PL "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Revolution            PM "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Reparation            PY "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Echos ECHOS "Echos", "echos.png",
"Tout effet appliqué lors de ce tour sera appliqué une seconde fois de suite." }

        { Pouvoir Memoire MEMOIRE "Mémoire", "Mémoire.png",
"Lorsque cette carte est jouée, son nom et son effet deviennent celui d'une carte ciblée dans votre Aether." }
        { Pouvoir Souvenirs SOUVENIRS "Souvenirs", "Souvenirs.png",
"Ajoutez autant de cartes de votre Aether à votre main que possible jusqu'à ce que vous ayez 4 cartes en main." }
        { Pouvoir Nostalgie NOSTALGIE "Nostalgie", "Nostalgie.png",
"Jouez jusqu'à deux permanents depuis votre Aether, et l'alliance adverse joue jusqu'à un permanent depuis son Aether." }

        { Pouvoir Acquisition           PH "Sans nom", "_.png",
"Chaque joueur pioche 2 cartes et défausse des cartes jusqu'à en avoir au maximum 2 en main." }
        { Pouvoir Elan                  PO "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Inertie               PN "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Cascade               PP "Sans nom", "_.png", "Sans effet" }

        { Entite  LaTechnique           EZ "Momo La Technique", "_.png", "Sans effet" } 
        { Entite  LeSceptique           EA "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaRigide              EB "Sans nom", "_.png", "Sans effet" } 
        { Entite  LHeritage             EC "Sans nom", "_.png", "Sans effet" } 
        { Entite  ConceptALAccumulateur ED "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaConservatrice       EE "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeGardien             EF "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaForce               EG "Sans nom", "_.png", "Sans effet" } 
        { Entite  LEnseignante          EH "Sans nom", "_.png", "Sans effet" } 
        { Entite  LArchiviste           EI "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaBibliotheque        EJ "Sans nom", "_.png", "Sans effet" } 
        { Entite  LEncyclopedie         EK "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeConteur             EL "Sans nom", "_.png", "Sans effet" } 
        { Entite  LePerdant             EM "Sans nom", "_.png", "Sans effet" } 
        { Entite  LAcharne              EN "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaMagicienne          EO "Sans nom", "_.png", "Sans effet" } 
        { Entite  LAncien               EP "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaTour                EQ "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaDedication          ER "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeMonde               ES "Sans nom", "_.png", "Sans effet" } 
    }
}

// But: Beaucoup faire chier, et dominer le monde.
// - Effets abusés, mais à coûts élevés ou conditions exigeantes
pub mod nuit {
    use super::*;
    liste_des_cartes!{
        { Pouvoir Transmutation TRANSMUTATION "Transmutation", "Transmutation.png",
"Ciblez une carte dans un seul des endroits suivants: Le dessus d'un deck, le terrain, ou une main.
Ensuite, déclarez un nom de carte (vous pouvez aussi inventer ce nom).
Le nom de la carte ciblée est désormais remplacé par le nom que vous avez déclaré, jusqu'à ce qu'elle se retrouve dans l'Aether.

A tout moment, tout joueur de votre alliance peut activer l'effet de Transmutation depuis l'Aether en défaussant 1 carte.
" }
        { Pouvoir Manipulation MANIPULATION "Manipulation", "Manipulation.png", 
"Ciblez une carte d'une main adverse: L'alliance adverse ne peut pas jouer d'autres cartes tant que la cible est dans une main." }
        { Pouvoir Mort MORT "Mort", "mort.png",
"Appliquez un ou deux des effets suivants:
- Ciblez une carte d'une main adverse: l'adversaire la défausse.
- Ciblez une carte sur le terrain: envoyez-la dans l'Aether." }
        { Pouvoir Anarchie              PY "Sans nom", "_.png",
"Renvoyez tous les permanents dont vous n'êtes pas propriétaire au-dessus de leurs decks respectifs, dans l'ordre que vous choisissez." }
        { Pouvoir Devastation           PU "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Decimation            PV "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Catastrophe           PC "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Contrainte            PE "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Controle              PG "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Obstination           PD "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Determination         PL "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Desir                 PK "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Trac                  PM "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Deception             PJ "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Teleport              PH "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Isolation             PN "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Desintegration        PF "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Exclusion             PO "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Domination            PP "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Tentation             PQ "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Unicite               PR "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Fusion                PS "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Fission               PT "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Sabotage              PW "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Denonciation          PX "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Oubli OUBLI "Oubli", "Oubli.png",
"Remettez jusqu'à 10 cartes de l'Aether de votre alliance dans leurs decks respectifs." }
        { Pouvoir Trahison              P0 "Sans nom", "_.png",
"Lors de l'application de l'effet de cette carte, les joueurs adverses ne doivent pas pouvoir voir l'action de leur partenaire (fermer les yeux peut suffire).
Simultanément, chaque joueur adverse prend la carte du dessus de son deck et la pose soit face verso, soit face recto.
Ensuite, si l'une de ces deux cartes posées est face verso mais pas l'autre, le joueur qui a posé la carte face verso défausse toute sa main et envoie tous ses permanents dans l'Aether.
Sinon, les deux joueurs adverses défaussent 3 cartes et envoie un permament chacun dans l'Aether." }
        { Pouvoir Suicide               P1 "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Superiorite           P2 "Sans nom", "_.png", "Sans effet" }
        { Pouvoir Inversion             P4 "Sans nom", "_.png", "Sans effet" }
        { Entite  LePilote              EA "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeDemon               EB "Sans nom", "_.png", "Sans effet" } 
        { Entite  LAveugle              EC "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeRegne               ED "Sans nom", "_.png", "Sans effet" } 
        { Entite  LEspace               EE "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaLune                EF "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeSysteme             EG "Sans nom", "_.png", "Sans effet" } 
        { Entite  LaGalaxie             EH "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeCosmos              EI "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeTrouNoir            EJ "Sans nom", "_.png", "Sans effet" } 
        { Entite  LeVortex              EK "Sans nom", "_.png", "Sans effet" } 
        { Entite  ConceptDLeDestructeur EL "Sans nom", "_.png", "Sans effet" } 
        { Entite  XLInconnue            EM "Sans nom", "_.png",
"* Cette fée était une cosmologue. Suite à une conclusion fataliste sur le but de l'existence, son coeur a formé un trou noir et inversé son corps." } 
        { Entite  YLeSummum             EN "Sans nom", "_.png",
"Lorsque cette carte arrive sur le terrain, ciblez un joueur adverse.
Vous dominez ce joueur tant que l'Aether de votre alliance contient strictement plus de cartes que l'Aether de l'alliance adverse.
Au début du tour adverse, chaque joueur adverse doit choisir une carte de son Aether et l'ajouter à sa main.

* Un ego à la hauteur de ses accomplissements." } 
        { Entite  ZLesProfondeurs       EO "Sans nom", "_.png", "Sans effet" } 
        { Entite  TLeTemps              EP "Sans nom", "_.png",
"Vous ne pouvez jouer cette carte que s'il y a 10 cartes ou plus dans un des Aethers.

Au début de votre tour, faites pivoter cette carte de 90° vers la droite.
A ce moment, si elle a effectué une rotation complète depuis son arrivée sur le terrain, vous gagnez la partie (c'est-à-dire que vous gagnez après que 4 tours soient passés en présence de cette carte)." } 
        { Entite  ELaSuprematie         EQ "e: La Suprématie", "_.png",
"Lorsque cette carte arrive sur le terrain, ciblez un joueur adverse.
Vous dominez ce joueur tant que vous avez strictement plus de cartes en main.
Au début de votre tour, vous pouvez piocher 1 carte en plus de votre pioche normale.

* L'exponentielle domine toute puissance antagoniste." } 
        { Entite  TauLaBoucle           ER "τ: La Boucle", "_.png",
"Toute carte devant aller dans l'Aether retourne sur le dessus du deck de son propriétaire à la place.
Si cela affecte plusieurs cartes simultanément, vous choisissez l'ordre.

\"$self\" doit être envoyée dans l'Aether dès que votre allié joue une carte.

* τ = 2π" }
        { Entite  KLeStatusQuo          ES "Sans nom", "_.png", "Sans effet" }
        { Entite  DeltaLeChangement     ET "Sans nom", "_.png", "Sans effet" }
        { Entite  ZeroLeNeant           EU "Sans nom", "_.png", "Les cartes n'ont plus de nom. Tout effet qui mentionne un nom ne peut donc plus s'appliquer, et la condition de victoire de l'Aube est par conséquent inatteignable tant que cette carte est sur le terrain." }
        { Entite  UnLExistence          EV "Sans nom", "_.png", "Sans effet" }
        { Entite  DeuxLaVie             EW "Sans nom", "_.png", "Sans effet" }
        { Entite  TroisLePlan           EX "Sans nom", "_.png", "Sans effet" }
        { Entite  QuatreLeSolide        EY "Sans nom", "_.png", "Sans effet" }
        { Entite  InfiniteLUnivers      EZ "Sans nom", "_.png", "Si une carte doit aller dans l'Aether, elle est placée en dessous du deck à la place." }
    }
}



fn main() {

}
