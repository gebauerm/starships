# 🚀 Starships

A fast-paced 2-player local multiplayer spaceship battle game built with Rust & Bevy.


---

## About

Starships is a top-down space combat arena where two players face off against each other in classic arcade-style action. Each player controls a unique ship with full movement freedom, shooting mechanics and physics-based flight.

Built from the ground up using the Bevy game engine, this project demonstrates modern Rust game development patterns including ECS architecture, proper system ordering, collision detection and input handling.

---

## ✨ Features

- ✅ Local 2-player split controls
- ✅ Physics based movement with thrust and rotation
- ✅ Projectile shooting system with cooldowns
- ✅ Screen wrapping boundaries
- ✅ Health & damage system
- ✅ Collision detection
- ✅ Pixel perfect sprite rendering
- ✅ 60 FPS fixed timestep game loop

---

## 🎮 Controls

| Action            | Player 1 (Defender) | Player 2 (Attacker) |
|-------------------|---------------------|---------------------|
| Thrust Forward    | `W`                 | `↑` Arrow Up        |
| Reverse / Brake   | `S`                 | `↓` Arrow Down      |
| Rotate Left       | `A`                 | `←` Arrow Left      |
| Rotate Right      | `D`                 | `→` Arrow Right     |
| Fire Weapon       | `Space`             | `Numpad 0`          |

---

## ⚙️ Game Mechanics

- Each ship starts with 100 health points
- Shots deal damage per hit
- Fire rate is delayed
- There are currently no damage multipliers or armour mechanics

---

## 🚀 Getting Started

### Prerequisites
You will need:
- Rust 1.75+ (2024 edition)
- Cargo package manager

### Running the game

```bash
# Clone the repository
git clone https://github.com/gebauerm/starships.git
cd starships

# Run the game
cargo run
```

The game window will open automatically. Grab a friend and start battling!

For better performance you can run with release optimizations:
```bash
cargo run --release
```

---

## 🛠 Built With

- [Rust](https://www.rust-lang.org/) - Programming Language
- [Bevy Engine](https://bevyengine.org/) v0.18.1 - Game Engine
- Entity Component System architecture

---

## 🗺 Roadmap

Planned upcoming features:
- [ ] Boost system for movement
- [ ] Floating obstacles / asteroids
- [ ] Score system & win conditions
- [ ] Sound effects
- [ ] Particle effects
- [ ] Damage based on hit location
- [ ] Weapon powerups
- [ ] Game over screen

---

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check the issues page.

---

## ✍️ Author

**elysias**

---

## 📝 License

This project is open source and available under the MIT License.

---

⭐ If you enjoyed this game, give this repository a star!
