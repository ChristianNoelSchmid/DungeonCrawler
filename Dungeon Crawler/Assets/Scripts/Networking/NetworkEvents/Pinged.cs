using System;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class Pinged : NetworkEvent 
    {
        public string CreateString() => "Pinged";
    }
}