# Registro de Alterações (Changelog)

Todas as alterações notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

---

## [0.2.0] - 2026-11-19

### 🚀 Paridade Visual Oficial Telegram Desktop, Bandeja (Tray) e Localização 100% PT-BR

Segunda grande versão estável com paridade de design idêntica ao Telegram Desktop e melhorias fundamentais de usabilidade.

#### Adicionado
- **Design Idêntico ao Telegram Desktop:**
  - Cabeçalho com botão hambúrguer (☰) na extrema esquerda abrindo o menu lateral.
  - Barra de pesquisa centralizada em formato de pílula (`#242f3d`) com texto *"Pesquisar"*.
  - Abas de pastas de conversas horizontais limpas com indicador azul inferior (`#2481cc`) e contadores de mensagens não lidas.
  - Paleta de cores oficial do Modo Escuro do Telegram Desktop (`#0e1621` fundo geral, `#17212b` painel lateral, `#182533` mensagens recebidas, `#2b5278` mensagens enviadas, `#2481cc` destaque e seleção).
  - Balões de mensagem com cantos arredondados e sombras suaves no padrão oficial.
- **Suporte Nativo a Bandeja do Sistema (System Tray):**
  - Implementado protocolo D-Bus `StatusNotifierItem` compatível com KDE Plasma, GNOME e XFCE.
  - Suporte a ocultar para a bandeja ao fechar a janela principal e restaurar com um clique.
- **Localização 100% em Português do Brasil (PT-BR):**
  - Todos os diálogos, menus, preferências e cabeçalhos 100% traduzidos (353 mensagens no catálogo).
  - Seletor multilíngue dinâmico na aba Idiomas das preferências com 23 idiomas suportados.
- **Gaveta Oficial do Telegram (Drawer Lateral):**
  - Atalhos diretos para Contas, Chats Arquivados, Contatos, Mensagens Salvas, Modo Noturno e Configurações.
- **Branding RicardTech:** Identidade PaperPlane Next configurada no diálogo Sobre e links oficiais da comunidade.

---

## [0.1.0] - 2026-11-17

### ✈️ Renascimento Comunitário (Community Revival)

Primeira versão da nova fase do projeto sob a governança da comunidade **RicardTech**.

#### Adicionado
- **Empacotamento Arch Linux (AUR):** Criada a receita oficial `packaging/arch/PKGBUILD` para o pacote `paperplane-next-git`.
- **Localização 100% em Português do Brasil (PT-BR):** Catálogo de tradução `po/pt_BR.po` verificado e completo (310 mensagens traduzidas).
- **Redundância Git Dual-Push:** Sincronização automática simultânea entre **GitHub** (`ricardtech/paperplane-next`) e **GitLab** (`ricard-tech/paperplane-next`).
- **Licença em Português:** Adicionado o documento `LICENCA.pt_BR.md` com a versão da GNU GPLv3 para o público lusófono.
- **Painel em Tempo Real:** Integração com o Live Progress Dashboard para monitoramento transparente das fases de engenharia.

#### Alterado
- **Atualização da Toolchain:** Validação e compatibilidade com **Rust 1.80+ / 1.100**, **Meson 1.12+** e **Ninja 1.13+**.
- **Novo README:** Documentação modernizada com badges de status, instruções de instalação para distros baseadas em Arch Linux e guia de contribuição.
- **Mapeamento de Dependências:** Mapeamento explícito das bibliotecas nativas de aceleração gráfica e animações (`rlottie` e `telegram-tdlib` / `libtdjson`).
