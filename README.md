# IAM Lab

Aplicativo desktop em Rust para entender **Interações Atômicas e Moleculares**. A matéria começa por funções de onda e energia, avança por átomos e ligações e termina em modelos simples de sólidos.

## Estudar no aplicativo

Cada tema é apresentado em quatro passos curtos:

1. **Entender:** leia uma ideia por vez e abra a fórmula só depois da explicação.
2. **Prever e testar:** escreva o que espera ver, manipule a simulação e compare.
3. **Lembrar:** responda sem consultar e leia uma dica específica para a sua resposta.
4. **Explicar:** resolva uma prática no papel e anote o que entendeu.

Depois de uma resposta correta, o aplicativo traz a pergunta de volta após intervalos crescentes. As anotações, as previsões e o progresso ficam neste computador.

## Percurso

| Etapa | Tema | O que observar |
|---|---|---|
| Fundamentos | Autovalores e energia | Por que algumas ondas têm energia definida |
| Fundamentos | Probabilidade | Como a área de `|ψ|²` vira chance |
| Fundamentos | Superposição | Como duas ondas interferem e mudam no tempo |
| Estados ligados | Oscilador harmônico | Por que o estado de menor energia não é zero |
| Estados ligados | Caixa tridimensional | Como estados diferentes podem ter a mesma energia |
| Átomos | Orbitais de hidrogênio | Como forma, nós e probabilidade se relacionam |
| Momento angular e spin | Momento angular orbital | Como módulo e projeção diferem |
| Momento angular e spin | Stern–Gerlach | Como um gradiente magnético separa um feixe |
| Momento angular e spin | Efeito Zeeman | Como spin–órbita e campo magnético separam níveis |
| Muitos elétrons | Pauli, Hund e campo médio | Como ocupações e repulsão mudam o átomo |
| Moléculas e sólidos | Ligações moleculares | Como fase e ocupação afetam uma ligação |
| Moléculas e sólidos | Redes e bandas | Como muitos átomos formam bandas de energia |

As cenas tridimensionais mostram amostras estáticas de probabilidade. A cor das nuvens representa o sinal da função de onda; cada ponto não é uma trajetória nem um elétron adicional. As aproximações aparecem junto da simulação e em [limites dos modelos](docs/arquitetura.md).

## Executar

Instale Rust estável em <https://rustup.rs/>. No Windows, instale também as ferramentas C++ e o Windows SDK do Visual Studio.

```sh
cargo run --release --locked
```

Na primeira compilação, Cargo baixa as dependências. A fonte matemática Noto Sans Math e sua licença estão incluídas no repositório. O executável fica em `target/release/iam-lab.exe`; o GitHub Actions publica uma cópia na aba Actions.

## Código

`src/course/` organiza explicações e práticas; `src/physics/` mantém os modelos sem dependência de interface; `src/views/` liga controles às simulações; `src/render/` desenha as cenas 3D; `src/app/` cuida da navegação e do estudo local. Consulte [a trilha conceitual](docs/trilha.md), [as escolhas pedagógicas e suas fontes](docs/aprendizagem.md) e [a arquitetura científica](docs/arquitetura.md).

## Compilação

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
```
