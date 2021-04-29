using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace DungeonCrawler.Monobehaviours
{
    public class Obstacles : MonoBehaviour
    {
        private static bool _initialized = false;

        // Store taken positions as Vector2Ints, so that they will be
        // stored as structures rather than Monobehaviours (classes)
        private static HashSet<Vector2Int> _vectorPositions;

        // Store GridPositions as references, to ensure that the position
        // is removed from _takenPosition in the event that the object moves
        private static Dictionary <Transform, Vector2Int> _objectPositions;

        void Awake()
        {
            if(_initialized)
                Debug.LogError("Warning. Multiple Obstacles Monobehaviours found. There should only be one per scene.");

            _initialized = true;

            _vectorPositions = new HashSet<Vector2Int>();
            _objectPositions = new Dictionary<Transform, Vector2Int>();
        }

        public static bool UpdateObstacle(Transform tr, Vector2Int pos)
        {
            if(_vectorPositions.Contains(pos))
                return false;

            if(_objectPositions.ContainsKey(tr))
            {
                _vectorPositions.Remove(_objectPositions[tr]);
                _objectPositions.Remove(tr);
            }
                
            _objectPositions.Add(tr, pos);
            _vectorPositions.Add(pos);

            return true;
        }

        public static bool RemoveObstacle(Transform tr)
        {
            if(_objectPositions.ContainsKey(tr))
            {
                _vectorPositions.Remove(_objectPositions[tr]);
                _objectPositions.Remove(tr);
                return true;
            }
            return false;
        }
    }
}