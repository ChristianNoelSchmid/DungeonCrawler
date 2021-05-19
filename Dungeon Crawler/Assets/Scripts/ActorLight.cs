using UnityEngine;

using DungeonCrawler.Monobehaviours;

namespace DungeonCrawler.Monobehaviours
{
    public class ActorLight : MonoBehaviour
    {
        private static LightGenerator _generator;

        [SerializeField]
        private int _range;

        [SerializeField]
        private float _intensity;

        private float _currentIntensity;

        public int Range { get => _range; set => _range = value; }
        public float Intensity => _currentIntensity;

        private bool _enabled = true;
        public bool Enabled
        {
            get => _enabled;
            set
            { 
                _enabled = value;
                _currentIntensity = _enabled ? _intensity : 0.0f;
                _generator.UpdateLight(this);
            }
        }


        private GridPosition _position;
        public GridPosition Position => _position;
        private Vector2Int _previousPosition = new Vector2Int(-1, -1);

        private void Awake()
        {
            if(_generator == null)
                _generator = FindObjectOfType<LightGenerator>();
            _position = GetComponent<GridPosition>();
            _currentIntensity = _intensity;
        }

        private void Start()
        {
            Enabled = true;
        }

        private void Update()
        {
            if(_previousPosition != _position.Value)
                _generator.UpdateLight(this);
            _previousPosition = _position.Value;
        }

        private void OnDestroy()
        {
            _currentIntensity = 0.0f;
            _generator.UpdateLight(this);
        }
    }
}