using UnityEngine;
using System;
#if UNITY_EDITOR
using UnityEditor;
#endif

/// <summary>
/// A convenience static class for exitting the game quickly, both in the Editor and the final game.
/// </summary>
public static class Exit {
    /// <summary>
    /// Exit the game (in the Editor, exits Play Mode).
    /// </summary>
    // NOTE: I would have named this method Exit() instead, but C# won't allow me.
    public static void Now(string reason="") {
        if (!string.IsNullOrEmpty (reason)) {
            // NOTE: Don't throw an exception, otherwise we won't reach the Exit.Now() line.
            Debug.LogError("Exitting: " + reason + Environment.NewLine + Environment.StackTrace);
        }
#if UNITY_EDITOR
        EditorApplication.isPlaying = false;
#endif
        Application.Quit ();
    }
    /// <summary>
    /// Exit if a condition is met.
    /// </summary>
    public static void If(bool doIt, string reason="") {
        if (doIt)
            Exit.Now (reason);
    }
}
