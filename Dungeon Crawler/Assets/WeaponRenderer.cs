using Assets.Scripts;
using System;
using System.Collections;
using UnityEngine;

public class WeaponRenderer : MonoBehaviour, IRenderer
{
    [SerializeField]
    private Sprite _equippedSprite;

    [SerializeField]
    private Sprite _actionSprite;

    private readonly WaitForSeconds _waitForFrameInterval = new WaitForSeconds(0.1f);

    private SpriteRenderer _renderer;

    public void TriggerAction(ActionType actionType)
    {
        switch(actionType)
        {
            case ActionType.PrimaryPressed:
                StartCoroutine(
                    CommitAction(
                        () => _renderer.sprite = _actionSprite,
                        () => _renderer.sprite = _equippedSprite
                    )
                );
                break;
            case ActionType.SecondaryHeld:
                _renderer.sprite = _actionSprite;
                break;
            case ActionType.SecondaryReleased:
                _renderer.sprite = _equippedSprite;
                break;
        }
    }

    private void Awake() => _renderer = GetComponent<SpriteRenderer>();

    private IEnumerator CommitAction(Action action, Action after)
    {
        action();
        yield return _waitForFrameInterval;
        after();
    }

}
