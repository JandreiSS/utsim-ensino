# Changelog

As mudanças deste repositório ficam registradas aqui. O histórico de antes da separação
(2026-11-26) está no CHANGELOG.md da família.

## [Não lançado]

## [ensino-0.27.0] - 2026-11-27

### Novo
- UTSim Ensino: pacote de missões "Fábrica verde" (UTSim Ambiental, AM12), com quatro casos de uma linha de pintura: a energia gasta sem produzir, a mochila de impacto do refugo, o destino dos resíduos e o compromisso entre custo e carbono. As missões leem as métricas ambientais de cada nó, que a família 0.2.1 publica no relatório. Os fatores são didáticos.

### Alterado
- Usa a família pela tag v0.2.1.

### Corrigido
- `scripts/publicar-ensino.sh`: os pacotes de missões e os elementos 3D vêm da família (o checkout da tag do Cargo.toml, ou `UTSIM_FAMILIA`). Antes, o script procurava `ensino/pacotes` e `assets/modelos3d` neste repositório, que não os tem desde a separação.

### Alterado
- Repositório próprio, separado do monorepo da família UTSim; usa a família pela tag v0.1.0.
