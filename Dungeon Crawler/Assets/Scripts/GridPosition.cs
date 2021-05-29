using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Monobehaviours 
{
    public class GridPosition : MonoBehaviour
    {
        [SerializeField]
        public bool _canFlip = true;
        private Transform _transform;

        public Vector2Int Value { get; set; }

        private Direction _direction;
        public Direction Direction 
        {
            get => _direction;
            set 
            {
                _direction = value;
                if(_canFlip)
                {
                    float x = _direction switch {
                        Direction.Right => 1.0f,
                        Direction.Left => -1.0f,
                        _ => 1.0f,
                    };

                    _transform.localScale = new Vector3(x, 1.0f, 1.0f);
                }
            }
        }

        private void Awake() => _transform = transform;
        
        public TransformModel ToTransformModel() => new TransformModel
        {
            X = Value.x,
            Y = Value.y,
            Direction = Direction
        };

        public void FromTransformModel(TransformModel transform)
        {
            Value = new Vector2Int(transform.X, transform.Y);
            Direction = transform.Direction;
        }
    }
}