using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents
{
    public class Hello : NetworkEvent
    {
        public Player Player { get; set; }
        public string CreateString() => $"Sync::Hello::{Player.Name}";
    }
}