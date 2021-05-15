namespace DungeonCrawler.Models
{
    public enum Direction {
        Left = 0,
        Right = 1,
    }

    public class PositionModel : ISerializable
    {
        public int X { get; set; }
        public int Y { get; set; }
        public Direction Direction { get; set; }

        public string Serialize() => $"{X}::{Y}::{(int)Direction}";
    }
}