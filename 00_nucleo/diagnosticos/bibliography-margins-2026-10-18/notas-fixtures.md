# Verificação dos testes antes da implementação

A primeira compilação de fixtures exigiu anotação f64 no cálculo do delta.
Não foi contada como RED funcional. A segunda rodada confirmou os dois REDs
de Block; os dois testes de Heading pararam antes da causa porque assumiam
que Layouter::new(...,12.0,...) instalava fonte12. O contrato atual de mod.rs
explica que esse argumento de geometria é ignorado e a chain começa em11.
As fixtures passaram a capturar/restaurar o tamanho causal e usar11 no controle
de delta. Os snapshots e logs anteriores foram preservados. O RED funcional
válido de Heading é a rodada red3, antes de qualquer alteração produtiva.

O primeiro gate V26 também bloqueou antes do resselo: referências sem ordenação byte e pin de núcleo calculado sem o domínio de dependências. A lista foi ordenada e o pin refez o SHA-256 com o domínio TEKT-NUCLEUS-DEPS-V1. O gate foi mantido; --fix-hashes não foi executado sobre grafo inválido.
