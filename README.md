# IAM Lab

Aplicativo em Rust para acompanhar **Interações Atômicas e Moleculares, BCK0104-15, turma 2026.3 da UFABC**. A trilha segue o cronograma do professor Michel Mendoza e organiza cada aula em: **Entender**, **Explorar**, **Conferir** e **Caderno**. Há exemplos resolvidos, previsão antes da simulação, feedback em cada alternativa e anotações locais.

## Executar

Instale Rust estável em <https://rustup.rs/>. No Windows também são necessárias as ferramentas C++ e o Windows SDK do Visual Studio.

```sh
cargo run --release
```

Cargo baixa as dependências na primeira compilação; a fonte matemática Noto Sans Math e sua licença já vêm no repositório. O executável Windows fica em `target/release/iam-lab.exe`. O GitHub Actions também gera um build para download na aba Actions.

## Comece a estudar

Abra **Meu percurso** e comece por **O que uma equação de autovalores diz?**. As 12 aulas avançam gradualmente, com um glossário para função, derivada, integral, fase, valor esperado e unidades. Pode abrir outro assunto pelo índice lateral.

| Aula | Experimento |
|---|---|
| Autovalores | Compara ψ e Hψ |
| Probabilidade | Poço 1D, integral num intervalo, nós e valores esperados |
| Superposição | Anima a interferência e calcula a energia do fóton |
| Oscilador | Estados de Hermite, níveis e energia de ponto zero |
| Caixa 3D | Nuvem rotacionável e degenerescência |
| Hidrogênio | Orbitais 1s, 2s e 2p, corte e probabilidade radial |
| Momento angular | Módulo e projeções Lz |
| Stern–Gerlach | Forças e separação dos feixes |
| Zeeman | Acoplamento spin–órbita e desdobramento magnético |
| Átomos multieletrônicos | Diagramas de Pauli e Hund para H–Ar |
| Moléculas | Orbitais ligantes, ligações, dipolos e líquidos |
| Sólidos | Redes cristalinas, bandas e Fermi–Dirac |

As nuvens 3D mostram amostras estáticas de probabilidade. As cores indicam o sinal de ψ, não a carga; pontos não são trajetórias nem elétrons adicionais. As hipóteses e limites de cada modelo aparecem na aula. O progresso acompanha checagens corretas, mas não comprova domínio do tema.

## Estrutura

`src/app/` organiza navegação e UX; `src/course/` contém lições e feedback; `src/physics/` implementa os modelos sem depender da interface; `src/views/` monta controles; `src/render/` desenha as cenas 3D. `assets/fonts/` contém a fonte matemática e a licença. As anotações antigas são preservadas na migração da v0.1.

## Verificação

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
```

## Referências

- [Site da turma 2026.3 de Michel Mendoza](https://sites.google.com/site/xxmichelmendoza/intera%C3%A7%C3%B5es-at%C3%B4micas-e-moleculares-bck0104-15?authuser=0)
- [Cronograma e ementa da turma](https://drive.google.com/file/d/1OynQ46aJRfofeDTO9mGY8H-gkvkMALCe/view) · [Lista de exercícios](https://drive.google.com/file/d/1Bd__o5EInNnvlc5IL5g1mz-Rd99IJfCg/view)
- [Pesquisa PhET sobre desenho de interfaces para simulações](https://phet.colorado.edu/publications/archive/PhET%20Interview%20Paper%20Part%20II.htm)
- [Noto Sans Math e licença SIL OFL](https://github.com/google/fonts/tree/main/ofl/notosansmath)

O aplicativo e as checagens são materiais independentes de estudo; não são oficiais da UFABC nem substituem a lista do professor. Consulte [ementa e alinhamento](docs/ementa.md) e [arquitetura e aproximações](docs/arquitetura.md).
