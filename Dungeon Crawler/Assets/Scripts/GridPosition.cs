using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace DungeonCrawler.Monobehaviours 
{
    public class GridPosition : MonoBehaviour
    {
        public int X { get; set; }
        public int Y { get; set; }

        public Vector2Int Position => new Vector2Int(X, Y);
    }
}