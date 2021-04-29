using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents
{
    /// <summary>
    /// NetworkEvent, sent by Server, informing Clients
    /// that a particular Client has left the game.
    /// </summary>
 
    public class PlayerLeft : NetworkEvent 
    {
        public DataModel CallerInfo { get; set; }

        public PlayerLeft() => CallerInfo = null;
        public PlayerLeft(string value) 
        {
            var args = value.Split(new string[] { "::" }, System.StringSplitOptions.None);
            CallerInfo = new DataModel
            {
                Id = int.Parse(args[0]),
            };
        }

        public string CreateString() => $"PlayerLeft::{CallerInfo.Serialize()}";
    }
}