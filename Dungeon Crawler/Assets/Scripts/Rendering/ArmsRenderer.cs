using Assets.Scripts;
using DungeonCrawler.Monobehaviours;
using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ArmsRenderer : MonoBehaviour, IRenderer
{
    [SerializeField] private Sprite _leftArmUnequipped;
    [SerializeField] private Sprite _leftArm1Handed;
    [SerializeField] private Sprite _leftArmAction;

    [SerializeField] private Sprite _rightArmUnequipped;
    [SerializeField] private Sprite _rightArm1Handed;
    [SerializeField] private Sprite _rightArmAction;

    private Transform _transform;

    private SpriteRenderer _leftArmRenderer;
    private SpriteRenderer _rightArmRenderer;
    private Transform _leftArmTransform;
    private Transform _rightArmTransform;

    private WeaponRenderer _leftWeaponRenderer;
    private WeaponRenderer _rightWeaponRenderer;

    private readonly WaitForSeconds _waitForFrameInterval = new WaitForSeconds(0.1f);

    public void TriggerAction(ActionType actionType)
    {
        switch (actionType)
        {
            case ActionType.PrimaryPressed:
                _rightWeaponRenderer.TriggerAction(actionType);
                StartCoroutine(
                    CommitAction(
                        () =>
                        {
                            var angle =
                                PlayerMovement.Angle(_transform.position, PlayerMovement.MousePosition);

                            if (angle > Mathf.PI / 3.0f && angle < 2.0f * Mathf.PI / 3.0f)
                                _rightArmTransform.localRotation = Quaternion.Euler(0.0f, 0.0f, 90.0f);
                            else if (angle > -2.0f * Mathf.PI / 3.0f && angle < -Mathf.PI / 3.0f)
                                _rightArmTransform.localRotation = Quaternion.Euler(0.0f, 0.0f, 270.0f);
                            else
                                _rightArmTransform.localRotation = Quaternion.identity;

                            _rightArmRenderer.sprite = _rightArmAction;
                        },
                        () =>
                        {
                            _rightArmRenderer.sprite = _rightArm1Handed;
                            _rightArmTransform.rotation = Quaternion.identity;
                        }
                    )
                );                                                          break;

            case ActionType.SecondaryHeld:
                    var angle =
                       PlayerMovement.Angle(_transform.position, PlayerMovement.MousePosition);

                    if (angle > Mathf.PI / 3.0f && angle < 2.0f * Mathf.PI / 3.0f)
                        _leftArmTransform.localRotation = Quaternion.Euler(0.0f, 0.0f, 90.0f);
                    else if (angle > -2.0f * Mathf.PI / 3.0f && angle < -Mathf.PI / 3.0f)
                        _leftArmTransform.localRotation = Quaternion.Euler(0.0f, 0.0f, 270.0f);
                    else
                        _leftArmTransform.localRotation = Quaternion.identity;

                _leftWeaponRenderer.TriggerAction(actionType);
                _leftArmRenderer.sprite = _leftArmAction;                   break;

            case ActionType.SecondaryReleased:
                _leftArmTransform.rotation = Quaternion.identity;

                _leftWeaponRenderer.TriggerAction(actionType);
                _leftArmRenderer.sprite = _leftArm1Handed;                  break;
        }
    }

    private void Awake()
    {
        _transform = transform;
        _leftArmTransform = transform.Find("ArmLeft");
        _rightArmTransform = transform.Find("ArmRight");
        _leftArmRenderer = _leftArmTransform.GetComponent<SpriteRenderer>();
        _rightArmRenderer = _rightArmTransform.GetComponent<SpriteRenderer>();
        _leftWeaponRenderer = _leftArmTransform.GetComponentInChildren<WeaponRenderer>(); 
        _rightWeaponRenderer = _rightArmTransform.GetComponentInChildren<WeaponRenderer>(); 
    }

    private IEnumerator CommitAction(Action action, Action after)
    {
        action();
        yield return _waitForFrameInterval;
        after();
    }
}
