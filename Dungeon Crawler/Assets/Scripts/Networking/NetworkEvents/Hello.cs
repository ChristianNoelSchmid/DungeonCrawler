using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents
{
    public class Hello : NetworkEvent
    {
        public Player Player { get; set; }
        public string CreateString() => $"Hello::{Player.Name}";
    }
}