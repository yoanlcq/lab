using System;
using UnityEngine.Assertions;
using UnityEngine;

/// <summary>
/// Convenient extension methods for all object types.
/// </summary>
public static class ConvenienceExtensions {
    /// <summary>
    /// Performs an operation on this object if it is not null.
    /// </summary>
    public static void IfNotNull<T>(this T o, Action<T> a) {
        if (o != null)
            a (o);
    }
    /// <summary>
    /// Performs an operation on this object if it is not null, and returns the result.
    /// </summary>
    public static TResult IfNotNull<T,TResult>(this T o, Func<T, TResult> a) {
        return o != null ? a (o) : default(TResult);
    }
    /// <summary>
    /// Asserts that this object is not null.
    /// </summary>
    public static void AssertNotNull(this object o) {
        Assert.IsNotNull (o);
    }

    /// <summary>
    /// Performs an operation on this component, catching any MissingComponentException.
    /// </summary>
    public static void IfValidComponent<T>(this T o, Action<T> a) where T: Component {
        o.IfNotNull(c => { try { a(c); } catch(MissingComponentException) {} });
    }
    /// <summary>
    /// Performs an operation on this component, catching any MissingComponentException.
    /// </summary>
    public static TResult IfValidComponent<T, TResult>(this T o, Func<T, TResult> a) where T: Component {
        o.IfNotNull (c => { try { return a (c); } catch (MissingComponentException) {} return default(TResult);});
        return default(TResult);
    }
}
