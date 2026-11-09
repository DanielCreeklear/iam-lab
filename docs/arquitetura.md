# Arquitetura e limites dos modelos

## Partes do aplicativo

- `course/` contém as explicações, perguntas, exemplos resolvidos e práticas curtas. Não depende da interface.
- `physics/` contém os cálculos e as amostras numéricas. Não depende da interface nem do armazenamento.
- `views/` transforma controles em parâmetros e resultados em gráficos.
- `render/` desenha eixos, nuvens e cenas 3D rotacionáveis.
- `app/` organiza navegação, etapas, anotações e revisões salvas localmente.
- `main.rs` configura e inicia a janela.

Cada modelo fica no próprio arquivo para facilitar a leitura e a manutenção. As cenas 3D usam uma projeção ortográfica e um pintor leve, sem motor de jogo.

## Hipóteses dos experimentos

- **Poço 1D:** partícula entre paredes infinitas; comprimento em nm e energia em eV.
- **Superposição:** combina estados estacionários. A fase relativa muda a distribuição espacial; as probabilidades de energia do sistema isolado permanecem constantes. A tela calcula energia de fóton, não a taxa de emissão.
- **Oscilador:** estados analíticos de uma parábola; níveis separados por `ℏω`, com energia fundamental `ℏω/2`.
- **Caixa 3D:** paredes infinitas. A energia depende de `nx² + ny² + nz²`, permitindo estados diferentes com a mesma energia.
- **Hidrogênio:** estados 1s, 2s e 2p do modelo monoeletrônico não relativístico. A nuvem é amostrada de `|ψ|²`, limitada a um volume finito. A cor mostra o sinal da onda, não a carga.
- **Momento angular:** autovalores e projeções orbitais. As setas comparam valores possíveis; não são vetores clássicos medidos em todas as direções.
- **Stern–Gerlach:** feixe ideal de átomos neutros e campo com gradiente. A simulação ignora estrutura hiperfina e outros detalhes pequenos.
- **Zeeman e spin–órbita:** fator de Landé no regime de campo fraco. O controle usa um acoplamento ajustável e não calcula a estrutura fina completa do hidrogênio.
- **Muitos elétrons:** preenchimento didático de H a Ar conforme Aufbau, Pauli e Hund. O campo médio é explicado, mas o aplicativo não resolve Hartree autoconsistente nem todos os termos espectroscópicos.
- **Moléculas:** sobreposição de orbitais, ordem de ligação e interações moleculares são modelos qualitativos; não predizem por si sós energias experimentais.
- **Sólidos:** cristais são células finitas desenhadas em projeção ortográfica; bandas são dispersões ilustrativas de duas bandas. O aplicativo não calcula condutividade.
- **Líquido:** Lennard–Jones em unidades reduzidas `σ = ε = kB = 1`, com 36 partículas em 2D, imagem mínima e corte `2,5σ`. O tempo de simulação não corresponde a segundos físicos.

Os controles limitam os parâmetros aos intervalos que os modelos aceitam. Use cada visualização para testar uma relação física e faça a derivação e as contas também no papel.

## Dados e privacidade

Previsões, respostas, etapas de revisão e anotações ficam no armazenamento local do eframe. Os dados anteriores de anotações são preservados ao abrir a versão nova; a migração não marca os conceitos como aprendidos. O aplicativo não envia dados para serviços remotos.
