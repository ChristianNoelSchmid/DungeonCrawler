using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace DungeonCrawler.Monobehaviours
{
    public class ActorLighting : MonoBehaviour
    {
        private static LightGenerator _generator;
        private GridPosition _position;
        private SpriteRenderer _renderer;
        private Color _originalColor;
        void Awake()
        {
            if(_generator ==  null)
                _generator = GameObject.FindObjectOfType<LightGenerator>();

            _renderer = GetComponentInChildren<SpriteRenderer>();
            _position = GetComponent<GridPosition>();
            _originalColor = _renderer.color;
        }
        void Update()
        {
            float brightness = _generator.SquareBrightness(_position.Value);
            _renderer.color = Color.Lerp(Color.black, _originalColor, brightness);
        }
    }
}