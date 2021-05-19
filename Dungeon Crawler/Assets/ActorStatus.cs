using Assets.Scripts.Models;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using DungeonCrawler.Monobehaviours;

[RequireComponent(typeof(Animator), typeof(ActorLight))]
public class ActorStatus : MonoBehaviour
{
    private Status _status = Status.Active;
    public Status Status
    {
        get => _status; 
        set
        {
            _status = value;
            _light.Enabled = _status == Status.Active;
            _animator.SetTrigger(_status.ToString());
        }
    }

    private ActorLight _light;
    private Animator _animator;

    private void Awake()
    {
        _light = GetComponent<ActorLight>();
        _animator = GetComponent<Animator>();
    }
}
