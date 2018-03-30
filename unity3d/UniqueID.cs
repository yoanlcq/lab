/// <summary>
/// UniqueID is a curious template that has to be inherited by world entities
/// that are represented by a unique ID, such as Checkpoints in a game, for instance.
/// 
/// Only once when the game starts, it ensures that all IDs are indeed unique and valid.
/// For this, it uses FindObjectsOfType(), and doesn't discard the result: instead, it
/// stores it permanently in a static "All" Dictionary that maps IDs to objects.
/// This means that other scripts can look up objects of this type by their ID any time.
/// 
/// At its simplest, you can use it like so:
/// <code>
/// public class Foo: UniqueID<Foo> {}
/// // Foo is then a regular component that is granted unique ID superpowers.
/// </code>
/// </summary>

using System.Collections;
using System.Collections.Generic;
using System;
using UnityEngine;
using UnityEngine.Assertions;

public class UniqueID<T>: MonoBehaviour where T: UniqueID<T> {

    public int ID = 0; // Initially set this to a default invalid value.

    // Used to ensure uniqueness of IDs.
    static Dictionary<int, T> all = new Dictionary<int, T>();
    public static Dictionary<int, T> All {
        get { 
            if (all.Count == 0) {
                CollectAllOnce();
            }
            return all;
        }
    }

    public void AssertIDIsValid() {
        Assert.IsTrue (ID > 0, "\""+gameObject.name+"\": "+typeof(T).Name+" IDs must have a unique value! 0 or below is invalid.");
    }
    public static void CollectAllOnce() {
        if (all.Count > 0) // FindObjectsOfType() is expensive, ensure we do this only once.
            return;

        Debug.Log ("Collecting all instances of " + typeof(T).Name);
        var collected = FindObjectsOfType<T>();
        foreach (var c in collected) {
            try {
                all.Add(c.ID, c);
            } catch(ArgumentException) { // Key already present
                Assert.IsFalse(true,
                    "\"" + c.gameObject.name + "\": "+typeof(T).Name+" ID " + c.ID
                    + " is already taken by \"" + All [c.ID].gameObject.name + "\"! You must choose another."
                );
            }
        }
    }

    void Start() {
        AssertIDIsValid ();
        CollectAllOnce ();
    }
}
