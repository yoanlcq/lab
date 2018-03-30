// http://wiki.unity3d.com/index.php/Xbox360Controller
// TODO: Test XboxControllerInput on Mac OS X

using System;
using UnityEngine;

internal class MakeCSharpHappyException: Exception {
	public MakeCSharpHappyException(): base("This piece of code is unreachable but C# ain't clever enough to figure it out") {}
}
internal class InternalMisuseException: Exception {
	public InternalMisuseException() : base("Some internal functionality was misused; you should never see this") {}
	public InternalMisuseException(string msg) : base(msg + " (some internal functionality was misused; you should never see this)") {}
}

internal enum DPadMethod {
	AxisPair,
	FourButtons,
}
internal enum TriggerRange {
	ZeroToOne,
	NegativeOneToOne,
}

internal interface IXboxControllerInputMappings {
	KeyCode A { get; }
	KeyCode B { get; }
	KeyCode X { get; }
	KeyCode Y { get; }
	KeyCode LB { get; }
	KeyCode RB { get; }
	KeyCode Back { get; }
	KeyCode Start { get; }
	KeyCode LStickClick { get; }
	KeyCode RStickClick { get; }
	string LX { get; }
	string LY { get; }
	string RX { get; }
	string RY { get; }
	// Windows supports a 0 to 1 range for both triggers.
	// Mac OS X supports -1 to 1, however the trigger initially starts at 0 until it is first used.
	string LT { get; }
	string RT { get; }
	TriggerRange TriggersRange { get; }
	DPadMethod DPadMethod { get; }
	string DPadX { get; }
	string DPadY { get; }
	float DPadYFactor { get; }
	KeyCode DPadUp { get; }
	KeyCode DPadDown { get; }
	KeyCode DPadLeft { get; }
	KeyCode DPadRight { get; }
}

internal class XboxControllerInputMappingsOSX: IXboxControllerInputMappings {
	public KeyCode A { get { return KeyCode.JoystickButton16; } }
	public KeyCode B { get { return KeyCode.JoystickButton17; } }
	public KeyCode X { get { return KeyCode.JoystickButton18; } }
	public KeyCode Y { get { return KeyCode.JoystickButton19; } }
	public KeyCode LB { get { return KeyCode.JoystickButton13; } }
	public KeyCode RB { get { return KeyCode.JoystickButton14; } }
	public KeyCode Back { get { return KeyCode.JoystickButton10; } }
	public KeyCode Start { get { return KeyCode.JoystickButton9; } }
	public KeyCode LStickClick { get { return KeyCode.JoystickButton11; } }
	public KeyCode RStickClick { get { return KeyCode.JoystickButton12; } }
	public string LX { get { return "X Axis"; } }
	public string LY { get { return "Y Axis"; } }
	public string RX { get { return "Axis 3"; } }
	public string RY { get { return "Axis 4"; } }
	public string LT { get { return "Axis 5"; } }
	public string RT { get { return "Axis 6"; } }
	public TriggerRange TriggersRange { get { return TriggerRange.NegativeOneToOne; } }
	public DPadMethod DPadMethod { get { return DPadMethod.FourButtons; } }
	public KeyCode DPadUp { get { return KeyCode.JoystickButton5; } }
	public KeyCode DPadDown { get { return KeyCode.JoystickButton6; } }
	public KeyCode DPadLeft { get { return KeyCode.JoystickButton7; } }
	public KeyCode DPadRight { get { return KeyCode.JoystickButton8; } }
	public string DPadX { get { throw new InternalMisuseException ("D-pad is not an axis on Mac OS X"); } }
	public string DPadY { get { throw new InternalMisuseException ("D-pad is not an axis on Mac OS X"); } }
	public float DPadYFactor { get { throw new InternalMisuseException ("D-pad is not an axis on Mac OS X"); } }
}
internal class XboxControllerInputMappingsLinux: IXboxControllerInputMappings {
	public KeyCode A { get { return KeyCode.JoystickButton0; } }
	public KeyCode B { get { return KeyCode.JoystickButton1; } }
	public KeyCode X { get { return KeyCode.JoystickButton2; } }
	public KeyCode Y { get { return KeyCode.JoystickButton3; } }
	public KeyCode LB { get { return KeyCode.JoystickButton4; } }
	public KeyCode RB { get { return KeyCode.JoystickButton5; } }
	public KeyCode Back { get { return KeyCode.JoystickButton6; } }
	public KeyCode Start { get { return KeyCode.JoystickButton7; } }
	public KeyCode LStickClick { get { return KeyCode.JoystickButton9; } }
	public KeyCode RStickClick { get { return KeyCode.JoystickButton10; } }
	public string LX { get { return "X Axis"; } }
	public string LY { get { return "Y Axis"; } }
	public string RX { get { return "Axis 4"; } }
	public string RY { get { return "Axis 5"; } }
	public string LT { get { return "Axis 3"; } }
	public string RT { get { return "Axis 6"; } }
	public TriggerRange TriggersRange { get { return TriggerRange.ZeroToOne; } }
	public DPadMethod DPadMethod { get { return DPadMethod.AxisPair; } }
	// Wireless controllers only
	public KeyCode DPadUp { get { return KeyCode.JoystickButton13; } }
	public KeyCode DPadDown { get { return KeyCode.JoystickButton14; } }
	public KeyCode DPadLeft { get { return KeyCode.JoystickButton11; } }
	public KeyCode DPadRight { get { return KeyCode.JoystickButton12; } }
	// Wired controllers only
	public string DPadX { get { return "Axis 7"; } }
	public string DPadY { get { return "Axis 8"; } }
	public float DPadYFactor { get { return -1f; } }
}
internal class XboxControllerInputMappingsWindows: IXboxControllerInputMappings {
	public KeyCode A { get { return KeyCode.JoystickButton0; } }
	public KeyCode B { get { return KeyCode.JoystickButton1; } }
	public KeyCode X { get { return KeyCode.JoystickButton2; } }
	public KeyCode Y { get { return KeyCode.JoystickButton3; } }
	public KeyCode LB { get { return KeyCode.JoystickButton4; } }
	public KeyCode RB { get { return KeyCode.JoystickButton5; } }
	public KeyCode Back { get { return KeyCode.JoystickButton6; } }
	public KeyCode Start { get { return KeyCode.JoystickButton7; } }
	public KeyCode LStickClick { get { return KeyCode.JoystickButton8; } }
	public KeyCode RStickClick { get { return KeyCode.JoystickButton9; } }
	public string LX { get { return "X Axis"; } }
	public string LY { get { return "Y Axis"; } }
	public string RX { get { return "Axis 4"; } }
	public string RY { get { return "Axis 5"; } }
	public string LT { get { return "Axis 9"; } }
	public string RT { get { return "Axis 10"; } }
	public TriggerRange TriggersRange { get { return TriggerRange.ZeroToOne; } }
	public DPadMethod DPadMethod { get { return DPadMethod.AxisPair; } }
	public string DPadX { get { return "Axis 6"; } }
	public string DPadY { get { return "Axis 7"; } }
	public float DPadYFactor { get { return 1f; } }
	public KeyCode DPadUp    { get { throw new InternalMisuseException("D-pad is an axis on Windows"); } }
	public KeyCode DPadDown  { get { throw new InternalMisuseException("D-pad is an axis on Windows"); } }
	public KeyCode DPadLeft  { get { throw new InternalMisuseException("D-pad is an axis on Windows"); } }
	public KeyCode DPadRight { get { throw new InternalMisuseException("D-pad is an axis on Windows"); } }
}

internal class XboxControllerInputMappings: IXboxControllerInputMappings {
#if UNITY_EDITOR_WIN || UNITY_STANDALONE_WIN
	IXboxControllerInputMappings m = new XboxControllerInputMappingsWindows ();
#elif UNITY_EDITOR_LINUX || UNITY_STANDALONE_LINUX
	IXboxControllerInputMappings m = new XboxControllerInputMappingsLinux ();
#elif UNITY_EDITOR_OSX || UNITY_STANDALONE_OSX
	IXboxControllerInputMappings m = new XboxControllerInputMappingsOSX ();
#endif
	// NOTE: Take Func<> because merely getting the attribute may cause exceptions (it was designed that way)
	int getDPadValue(Func<string, float> fGetAxis, Func<float> factor, Func<string> axis, Func<KeyCode> negative, Func<KeyCode> positive) {
		switch(m.DPadMethod) {
		case DPadMethod.AxisPair:
			return Mathf.RoundToInt(factor() * fGetAxis(axis()));
		case DPadMethod.FourButtons:
			int v = 0;
			if (Input.GetKey (negative())) v -= 1;
			if (Input.GetKey (positive())) v += 1;
			return v;
		}
		throw new MakeCSharpHappyException();
	}
	public int GetDPadX() { return getDPadValue (Input.GetAxis, () => 1f, () => m.DPadX, () => m.DPadLeft, () => m.DPadRight); }
	public int GetDPadY() { return getDPadValue (Input.GetAxis, () => m.DPadYFactor, () => m.DPadY, () => m.DPadDown, () => m.DPadUp); }
	public int GetDPadXRaw() { return getDPadValue (Input.GetAxisRaw, () => 1f, () => m.DPadX, () => m.DPadLeft, () => m.DPadRight); }
	public int GetDPadYRaw() { return getDPadValue (Input.GetAxisRaw, () => m.DPadYFactor, () => m.DPadY, () => m.DPadDown, () => m.DPadUp); }

	float getTrigger(float v) {
		switch (m.TriggersRange) {
		case TriggerRange.ZeroToOne: return v;
		case TriggerRange.NegativeOneToOne: return .5f + v/2f;
		}
		throw new MakeCSharpHappyException ();
	}
	public float GetLT() { return getTrigger (Input.GetAxis(LT)); }
	public float GetRT() { return getTrigger (Input.GetAxis(RT)); }
	public float GetLTRaw() { return getTrigger (Input.GetAxisRaw(LT)); }
	public float GetRTRaw() { return getTrigger (Input.GetAxisRaw(RT)); }

	public KeyCode A { get { return m.A; } }
	public KeyCode B { get { return m.B; } }
	public KeyCode X { get { return m.X; } }
	public KeyCode Y { get { return m.Y; } }
	public KeyCode LB { get { return m.LB; } }
	public KeyCode RB { get { return m.RB; } }
	public KeyCode Back { get { return m.Back; } }
	public KeyCode Start { get { return m.Start; } }
	public KeyCode LStickClick { get { return m.LStickClick; } }
	public KeyCode RStickClick { get { return m.RStickClick; } }
	public string LX { get { return m.LX; } }
	public string LY { get { return m.LY; } }
	public string RX { get { return m.RX; } }
	public string RY { get { return m.RY; } }
	public string LT { get { return m.LT; } }
	public string RT { get { return m.RT; } }
	public TriggerRange TriggersRange { get { return m.TriggersRange; } }
	public DPadMethod DPadMethod { get { return m.DPadMethod; } }
	public string DPadX { get { return m.DPadX; } }
	public string DPadY { get { return m.DPadY; } }
	public float DPadYFactor { get { return m.DPadYFactor; } }
	public KeyCode DPadUp { get { return m.DPadUp; } }
	public KeyCode DPadDown { get { return m.DPadDown; } }
	public KeyCode DPadLeft { get { return m.DPadLeft; } }
	public KeyCode DPadRight { get { return m.DPadRight; } }
}

public enum XbAxis {
	DPadX, DPadY, LX, LY, RX, RY, LT, RT,
}
public enum XbAxis2D {
	DPad, LStick, RStick
}
public enum XbButton {
	A, B, X, Y, LB, RB, Back, Start, LStickClick, RStickClick,
}

public static class XboxControllerInput {
	static XboxControllerInputMappings m = new XboxControllerInputMappings ();

	static string axisToString(XbAxis axis) {
		switch(axis) {
		case XbAxis.LX: return m.LX;
		case XbAxis.LY: return m.LY;
		case XbAxis.RX: return m.RX;
		case XbAxis.RY: return m.RY;
		case XbAxis.LT: throw new InternalMisuseException ("The range of triggers is OS-specific and requires special handling");
		case XbAxis.RT: throw new InternalMisuseException ("The range of triggers is OS-specific and requires special handling");
		case XbAxis.DPadX: throw new InternalMisuseException ("On some OSes, D-pad isn't an axis");
		case XbAxis.DPadY: throw new InternalMisuseException ("On some OSes, D-pad isn't an axis");
		}
		throw new MakeCSharpHappyException ();
	}

	static KeyCode buttonToKeyCode(XbButton btn) {
		switch(btn) {
		case XbButton.A: return m.A;
		case XbButton.B: return m.B;
		case XbButton.X: return m.X;
		case XbButton.Y: return m.Y;
		case XbButton.LB: return m.LB;
		case XbButton.RB: return m.RB;
		case XbButton.Back: return m.Back;
		case XbButton.Start: return m.Start;
		case XbButton.LStickClick: return m.LStickClick;
		case XbButton.RStickClick: return m.RStickClick;
		}
		throw new MakeCSharpHappyException ();
	}

	public static bool GetButton(XbButton btn) { return Input.GetKey (buttonToKeyCode (btn)); }
	public static bool GetButtonDown(XbButton btn) { return Input.GetKeyDown (buttonToKeyCode (btn)); }
	public static bool GetButtonUp(XbButton btn) { return Input.GetKeyUp (buttonToKeyCode (btn)); }

	public static float GetAxis(XbAxis axis) {
		switch (axis) {
		case XbAxis.DPadX: return (float)m.GetDPadX ();
		case XbAxis.DPadY: return (float)m.GetDPadY ();
		case XbAxis.LT: return m.GetLT ();
		case XbAxis.RT: return m.GetRT ();
		case XbAxis.LY: return -Input.GetAxis (axisToString(axis)); // XXX: Assumes "Y Axis" doesn't have the "Inverted" checkbox checked.
		case XbAxis.RY: return -Input.GetAxis (axisToString(axis));
		default: return Input.GetAxis (axisToString(axis));
		}
	}
	public static float GetAxisRaw(XbAxis axis) {
		switch (axis) {
		case XbAxis.DPadX: return (float)m.GetDPadXRaw ();
		case XbAxis.DPadY: return (float)m.GetDPadYRaw ();
		case XbAxis.LT: return m.GetLTRaw ();
		case XbAxis.RT: return m.GetRTRaw ();
		case XbAxis.LY: return -Input.GetAxisRaw (axisToString(axis)); // XXX: Assumes "Y Axis" doesn't have the "Inverted" checkbox checked.
		case XbAxis.RY: return -Input.GetAxisRaw (axisToString(axis));
		default: return Input.GetAxisRaw (axisToString(axis));
		}
	}
	public static Vector2 GetAxis2D(XbAxis2D axis) {
		switch (axis) {
		case XbAxis2D.DPad: return new Vector2 (GetAxis(XbAxis.DPadX), GetAxis(XbAxis.DPadY));
		case XbAxis2D.LStick: return new Vector2 (GetAxis(XbAxis.LX), GetAxis(XbAxis.LY));
		case XbAxis2D.RStick: return new Vector2 (GetAxis(XbAxis.RX), GetAxis(XbAxis.RY));
		}
		throw new MakeCSharpHappyException ();
	}
	public static Vector2 GetAxis2DRaw(XbAxis2D axis) {
		switch (axis) {
		case XbAxis2D.DPad: return new Vector2 (GetAxisRaw(XbAxis.DPadX), GetAxisRaw(XbAxis.DPadY));
		case XbAxis2D.LStick: return new Vector2 (GetAxisRaw(XbAxis.LX), GetAxisRaw(XbAxis.LY));
		case XbAxis2D.RStick: return new Vector2 (GetAxisRaw(XbAxis.RX), GetAxisRaw(XbAxis.RY));
		}
		throw new MakeCSharpHappyException ();
	}
}
