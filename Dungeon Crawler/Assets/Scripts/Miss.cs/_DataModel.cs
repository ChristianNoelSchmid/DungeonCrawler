using System;

namespace DungeonCrawler.Models
{
    /* Serves as a wrapper for JSON objects being sent to and from server.  */
    public class DataModel : ISerializable
    {
        public int Id { get; set; }
        public string Serialize() => Id.ToString();
    }

    public class DataModel<T> : ISerializable
        where T : ISerializable
    {
        public int Id { get; set; }
        public T Value { get; set; }

        public string Serialize() =>
            $"{Id}::{Value.Serialize()}";
    }
}