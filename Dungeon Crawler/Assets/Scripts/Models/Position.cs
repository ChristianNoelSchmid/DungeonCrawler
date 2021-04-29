namespace DungeonCrawler.Models
{
    public class Position : ISerializable
    {
        public int X { get; set; }
        public int Y { get; set; }

        public string Serialize() => $"{X}::{Y}";
    }
}