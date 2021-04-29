using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Voxel : MonoBehaviour
{
    [SerializeField]
    private bool _isWall = false;
    public bool IsWall => _isWall;

    public SpriteRenderer Renderer { get; private set; }
    public bool LightUpdating { get; set; } = false;

    private void Awake()
    {
        Renderer = GetComponent<SpriteRenderer>();
        if(Renderer == null)
            Debug.LogError("Expected SpriteRenderer to be attached to Voxel, but none was found.");
    }
}
