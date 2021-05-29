using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace DungeonCrawler.Monobehaviours
{
    public class ActorLighting : MonoBehaviour
    {
        private static LightGenerator _generator;
        private GridPosition _position;
        private SpriteRenderer[] _renderers;
        void Awake()
        {
            if(_generator ==  null)
                _generator = FindObjectOfType<LightGenerator>();

            _renderers = GetComponentsInChildren<SpriteRenderer>();
            _position = GetComponent<GridPosition>();
        }
        void Update()
        {
            float brightness = _generator.SquareBrightness(_position.Value);
            for(int i = 0; i < _renderers.Length; ++i)
                _renderers[i].color = Color.Lerp(Color.black, Color.white, brightness);
        }
    }
}