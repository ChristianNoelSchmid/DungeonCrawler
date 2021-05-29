using Assets.Scripts;
using UnityEngine;

public class HumanoidRenderer : MonoBehaviour, IRenderer
{
    private ArmsRenderer _armsRenderer;
    private bool _secondaryHeld = false;
    
    // IRenderer.cs
    public void TriggerAction(ActionType actionType)
    {
        _armsRenderer.TriggerAction(actionType);
    }

    private void Awake() =>
        _armsRenderer = GetComponentInChildren<ArmsRenderer>();
}
