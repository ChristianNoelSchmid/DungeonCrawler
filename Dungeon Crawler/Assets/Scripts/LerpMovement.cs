using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace DungeonCrawler.Monobehaviours
{
    public class LerpMovement : MonoBehaviour
    {
        [SerializeField]
        private float _speed;

        private GridPosition _gridPosition;
        private Transform _transform;
        private void Awake()
        {
            _gridPosition = GetComponent<GridPosition>();
            if(_gridPosition == null)
                Debug.LogError("Expected GridPosition on MonoBehaviour, but it wasn't found.");

            _transform = transform;
        }

        void Update()
        {
            _transform.position = 
                Vector2.Lerp(_transform.position, _gridPosition.Value, Time.deltaTime * _speed);
        }
    }
}