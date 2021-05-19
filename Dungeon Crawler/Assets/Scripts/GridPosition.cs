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
        
        public PositionModel ToPositionModel() => new PositionModel
        {
            X = Value.x,
            Y = Value.y,
            Direction = Direction
        };

        public void FromPositionModel(PositionModel position)
        {
            Value = new Vector2Int(position.X, position.Y);
            Direction = position.Direction;
        }
    }
}