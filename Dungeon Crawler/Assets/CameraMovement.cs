using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraMovement : MonoBehaviour
{
    [SerializeField]
    private float _speed;

    [SerializeField]
    private Transform _target; 
    public Transform Target
    {
        get => _target;
        set => _target = value;
    }

    private Transform _transform;
    void Awake() => _transform = transform; 

    void Update() 
    {
        var position = Vector3.Lerp(
            _transform.position, 
            _target.position, 
            Time.deltaTime * _speed
        );

        position.z = _transform.position.z;

        _transform.position = position;
    }
}
