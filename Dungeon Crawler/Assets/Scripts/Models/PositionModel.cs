using UnityEngine;

namespace DungeonCrawler.Models
{
    public class PositionModel : ISerializable
    {
        public int X { get; set; }
        public int Y { get; set; }
        public string Serialize() => $"{X}::{Y}";
        public Vector2Int ToVector2Int() => new Vector2Int(X, Y);
        public static PositionModel FromVector2Int(Vector2Int value) => new PositionModel { X = value.x, Y = value.y };
    }
}