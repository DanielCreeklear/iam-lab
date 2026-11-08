# Arquitetura e limites dos modelos

## Organização do código

`course/` guarda os objetivos, explicações, exemplos resolvidos, perguntas de previsão e feedback dos questionários. O material pedagógico é dado estruturado, separado das janelas. `physics/` contém as funções e os modelos numéricos sem dependência de interface; `views/` liga controles a esses modelos; `render/` desenha nuvens, eixos e cenas 3D; `app/` controla navegação, estado de estudo, notas locais e tema. `main.rs` só configura e inicia a janela.

Cada domínio tem um arquivo próprio, como `confinement.rs`, `orbitals.rs`, `spin.rs` e `electrons.rs`. A organização mantém fórmulas, explicação e controles localizáveis sem criar uma camada genérica para cada tela.

## Como uma aula ensina

As 12 aulas seguem a sequência consultada no cronograma da turma: fundamentos e Schrödinger 1D; confinamento, probabilidades e transições; hidrogênio e momento angular; spin, Stern–Gerlach, Zeeman e acoplamento spin–órbita; átomos multieletrônicos; moléculas e sólidos como continuidade. Cada aula separa quatro passos: entender, explorar, conferir e registrar no caderno. Ela explicita pré-requisitos e símbolos, mostra um exemplo resolvido, pede uma previsão antes da simulação e apresenta uma questão curta com feedback por alternativa.

As respostas e o estágio de cada aula ficam no armazenamento local do eframe. Notas da versão 0.1 são migradas para o caderno sem marcar o conteúdo antigo como dominado. A janela não envia dados a serviços remotos. O progresso mostra revisões registradas, não uma medida de domínio.

## Hipóteses e unidades

- Poço infinito: partícula em 1D; comprimento em nm e energia em eV; n é inteiro positivo.
- Superposição: combinação de estados estacionários e batimento de densidade; a frequência da transição vem de ΔE/h. O movimento da densidade depende da fase relativa.
- Oscilador: os estados mostrados são soluções analíticas; os níveis são igualmente espaçados, incluindo energia de ponto zero.
- Caixa 3D: poço cúbico de paredes infinitas. A energia depende de `nx² + ny² + nz²`, por isso trincas distintas podem compartilhar um nível.
- Hidrogênio: orbitais monoeletrônicos não relativísticos 1s, 2s e 2p; coordenadas em raios de Bohr. Nuvens amostradas por `|ψ|²` são cortes estáticos num volume finito. As cores mostram o sinal da função, não carga; pontos não são trajetórias.
- Spin e Zeeman: projeções de spin ±1/2 e fator de Landé no limite de campo fraco. A visualização de Stern–Gerlach desenha a separação de pacotes de onda numa região magnética idealizada.
- Multieletrônicos: preenchimento didático H–Ar segundo Aufbau, Pauli e Hund. A ocupação não resolve a interação eletrônica nem todos os termos espectroscópicos.
- Moléculas: Morse, LCAO, dipolos e Lennard–Jones são modelos explicativos. Uma curva qualitativa não prevê, por si só, a energia experimental de uma molécula.
- Sólidos: cristais são desenhos de células finitas, projetados ortograficamente; coordenação e empacotamento descrevem o cristal infinito. Bandas são dispersões de brinquedo de duas bandas e não predizem a condutividade de um material.
- Líquido: Lennard–Jones em unidades reduzidas `σ = ε = kB = 1`, 36 partículas em 2D e dinâmica Metropolis com proposta simétrica, imagem mínima e corte `2,5σ`. A densidade e a temperatura não representam segundos nem uma substância específica.

Os controles limitam os parâmetros aos domínios definidos. As aproximações servem para explorar relações físicas e não substituem uma derivação, a resolução da lista da turma ou a conferência com fontes do curso.

## Referências e escopo do curso

O mapeamento aula a aula e os tópicos consultados estão em [ementa e alinhamento](ementa.md). A referência da turma é o [site do professor Michel Mendoza](https://sites.google.com/site/xxmichelmendoza/intera%C3%A7%C3%B5es-at%C3%B4micas-e-moleculares-bck0104-15?authuser=0), com o [cronograma 2026.3](https://drive.google.com/file/d/1OynQ46aJRfofeDTO9mGY8H-gkvkMALCe/view). O modelo Zeeman parte do fator de Landé para campo fraco; níveis e correções físicas variam conforme o regime e as aproximações consideradas.
