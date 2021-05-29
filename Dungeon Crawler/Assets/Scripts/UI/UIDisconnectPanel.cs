using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class UIDisconnectPanel : UIGroup
{
    protected override void Awake()
    {
        base.Awake();
        SetVisible(false);
    }
    public void Display() => SetVisible(true);
}
