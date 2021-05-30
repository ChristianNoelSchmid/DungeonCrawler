using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents
{
    /// <summary>
    /// NetworkEvent, sent by Server, informing Clients
    /// that a particular Client has left the game.
    /// </summary>
 
    public class PlayerLeft : NetworkEvent 
    {
        public DataModel Model { get; set; }

        public PlayerLeft() => Model = null;
        public PlayerLeft(string value) 
        {
            var args = value.Split(new string[] { "::" }, System.StringSplitOptions.None);
            Model = new DataModel
            {
                Id = int.Parse(args[0]),
            };
        }

        public string CreateString() => $"Sync::PlayerLeft::{Model.Serialize()}";
    }
}