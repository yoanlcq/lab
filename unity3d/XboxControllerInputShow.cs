using UnityEngine;

public class XboxControllerInputShow: MonoBehaviour {
	public bool A, B, X, Y, LB, RB, Back, Start, LStickClick, RStickClick;
	public Vector2 LStick, RStick, DPad;
	[Range(0f, 1f)] public float LT, RT;

	void Update() {
		A = XboxControllerInput.GetButton (XbButton.A);
		B = XboxControllerInput.GetButton (XbButton.B);
		X = XboxControllerInput.GetButton (XbButton.X);
		Y = XboxControllerInput.GetButton (XbButton.Y);
		LB = XboxControllerInput.GetButton (XbButton.LB);
		RB = XboxControllerInput.GetButton (XbButton.RB);
		Back = XboxControllerInput.GetButton (XbButton.Back);
		Start = XboxControllerInput.GetButton (XbButton.Start);
		LStickClick = XboxControllerInput.GetButton (XbButton.LStickClick);
		RStickClick = XboxControllerInput.GetButton (XbButton.RStickClick);
		LStick = XboxControllerInput.GetAxis2D (XbAxis2D.LStick);
		RStick = XboxControllerInput.GetAxis2D (XbAxis2D.RStick);
		DPad = XboxControllerInput.GetAxis2D (XbAxis2D.DPad);
		LT = XboxControllerInput.GetAxis (XbAxis.LT);
		RT = XboxControllerInput.GetAxis (XbAxis.RT);
	}
}
