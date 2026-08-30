# P1248 — auditar imagens SVG e formatos desconhecidos

**Estado:** FECHADO — MATERIALIZADO; CERTIFICADO FINAL PASS  
**Predecessor:** P1247  
**Saída:** matriz de preservação para raster, SVG embutido e formato Unknown.

Medir `FrameItem::Image` contra vanilla: tamanho intrínseco, aspect ratio,
transform, alpha, encoding, URI/data, SVG aninhado e comportamento de formato
desconhecido. Separar equivalência semântica de igualdade de bytes/base64.

Corrigir somente dentro do contrato existente e do owner L3, com L0 primeiro.
Não acessar rede, não inventar decoder e não aceitar omissão silenciosa como
paridade. Casos sem decoder ou dados suficientes recebem fallback/`Unknown`
documentado e atacável pela lente.

## Resultado anterior invalidado

A alegação de preparação isolada 6/6 e score 9/11 não era reproduzível: os
artefatos e o runner não estavam presentes. O recibo anterior fica invalidado.

## Execução saneada

O contrato corrigido possui oito observáveis de linguagem/morfologia: conteúdo
raster e alpha, caixa de layout, aspect ratio, orientação, clip de cover, SVG
aninhado visível, coerência do tipo de mídia e formato desconhecido. O oracle
possui seis casos `Preserved` e dois casos opacos `Unknown`.

Onze mutações negativas possuem testemunha, incluindo perda de forma visível e
alpha em SVG aninhado. O gate determinístico rejeita 11/11, score 1.0, e foi
repetido byte a byte. Nenhum candidato ou código produtivo foi executado.

A mesma sessão escreveu contrato, oráculos e ataques; portanto o resultado é
um presemantic gate reproduzível **sem atestação de isolamento**, não um preseal
segregado. A materialização continua proibida até repetir estes artefatos sob
autoridades realmente segregadas. P1248 pode ser fechado como saneamento, sem
alegar paridade de imagens SVG em produção.

## Materialização segregada final — 2026-08-28

O ensaio anterior foi substituído por contrato, oráculos, ataques e verificação
com autoridades segregadas por papel. O preseal revisado cobre 8 observáveis e
rejeita M01–M13, score `1.0`; a separação é processual em filesystem
compartilhado, sem alegação de isolamento ambiental forte.

O owner SVG agora:

- preserva data URL/MIME coerente para PNG, JPEG, GIF e WebP;
- reconhece conservadoramente SVG autossuficiente já transportado e o embute
  como `image/svg+xml`, sem parser ou novo enum público;
- consome `clip_rect` page-local e orientação EXIF exatamente uma vez;
- mantém fit/box resolvidos pelo layout, sem relayout no exporter;
- marca bytes desconhecidos, raiz ambígua, dependência externa e SVGZ como
  `Unknown` com `data-crystalline-image-fallback="unknown-format"`, sem
  inventar `<image>` ou MIME.

Certificação independente: 8/8 observáveis; M01–M13 rejeitados no nível das
saídas reais; 9/9 testes P1248; 36/36 suíte SVG; build, lint completo,
V5/V15/V26 e diff-check PASS. SVG aninhado foi extraído do data URL candidato,
rasterizado e amostrado com alpha; JPEG assimétrico orientação 6 e PNG cover
foram amostrados visualmente.

Certificado: `00_nucleo/diagnosticos/p1248-final-certificate.tsv`.
