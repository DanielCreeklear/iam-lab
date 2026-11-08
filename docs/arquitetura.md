# Arquitetura e critérios científicos

## Separação de responsabilidades

`physics` é uma biblioteca de funções puras, exceto o estado explícito do amostrador de líquidos. Não conhece egui, arquivos ou navegação. `views` traduz controles em parâmetros e resultados em gráficos. `app` cuida da escolha do tema e da persistência de notas e progresso. `main` apenas inicia a aplicação.

Evita-se uma hierarquia de traits para sete telas pequenas. O roteamento explícito facilita localizar e alterar cada experimento. Novos modelos devem ter seus próprios arquivos quando crescerem; compartilhe somente utilitários realmente comuns.

## Convenções e domínio

- Poço: elétron, nm e eV, n >= 1, L > 0.
- Hidrogenoide: r em a0, Z > 0; modelo monoeletrônico não relativístico.
- LCAO: orbitais 1s normalizados em três dimensões, R > 0; o corte axial não tem integral unitária.
- Morse: profundidade positiva em eV, distâncias em a0. Ilustração fenomenológica, sem inferir energias a partir da sobreposição.
- Dipolos: unidades reduzidas do prefator dipolar, ângulos em radianos no núcleo.
- Líquido: sigma=epsilon=kB=1; 0.15 <= densidade <= 0.85, T* > 0; 36 partículas em 2D. Metropolis sequencial com proposta simétrica, imagem mínima, cutoff 2.5 sigma e potencial deslocado. Caixa sempre maior que duas vezes o cutoff no domínio da interface. Não há relógio físico.
- Cristais: sítios em unidades do parâmetro de rede a; célula semiaberta e projeção ortográfica. Coordenação e empacotamento referem-se ao cristal infinito.
- Bandas: eV, ka adimensional, T > 0. Modelo de duas dispersões independentes; não prevê condutividade.

As funções são internas ao uso didático e pressupõem parâmetros válidos, impostos pelos controles. Uma futura API pública deve validar entradas e retornar erros para valores fora do domínio.

## Verificação

Testes verificam normalização e ortogonalidade, escalas de energia, nós e simetrias, mínimos de potencial, orientações de dipolos, limites periódicos e reprodutibilidade do fluido, contagem de sítios e ocupação de Fermi. Não substituem validação experimental ou revisão de especialista.

O CI compila em Windows, roda testes, rustfmt e Clippy e publica o executável como artefato da execução. O Cargo.lock deve ser versionado para builds reproduzíveis.

## Persistência

eframe salva notas, marcas de revisão e tema selecionado no diretório de dados local do usuário para IAM Lab. As simulações reiniciam com parâmetros padrão. Nenhum dado é enviado para serviços remotos pelo aplicativo.
