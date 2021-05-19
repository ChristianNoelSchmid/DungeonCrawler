using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[RequireComponent(typeof(CanvasGroup))]
public abstract class UIGroup : MonoBehaviour
{
    private CanvasGroup _group;

    private bool _isVisible;
    public bool IsVisible => _isVisible;

    protected virtual void Awake()
    {
        _group = GetComponent<CanvasGroup>(); 
    }

    public virtual void SetVisible(bool isVisible)
    {
        _isVisible = isVisible;
        _group.alpha = _isVisible ? 1.0f : 0.0f;
        _group.interactable = _isVisible;
        _group.blocksRaycasts = _isVisible;
    }
}
