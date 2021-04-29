using UnityEngine;

using DungeonCrawler.Monobehaviours;

namespace DungeonCrawler.Monobehaviours
{
    public class Light : MonoBehaviour
    {
        [SerializeField]
        private LightGenerator _generator;

        [SerializeField]
        private int _range;
        [SerializeField]
        private float _intensity;

        public int Range { get => _range; set => _range = value; }
        public float Intensity { get => _intensity; set => _intensity = value; }

        private GridPosition _position;
        public GridPosition Position => _position;
        private Vector2Int _previousPosition = new Vector2Int(-1, -1);

        private void Awake() => _position = GetComponent<GridPosition>();

        private void Update()
        {
            if(_previousPosition != _position.Position)
                _generator.UpdateLight(this);
            _previousPosition = _position.Position;
        }
    }
}