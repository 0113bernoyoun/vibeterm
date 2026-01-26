# VibeTerm 상세 스펙 문서 (PRD)

## 1. 프로젝트 개요

- **명칭:** VibeTerm (The Terminal for Vibe Coding)
- **개발 환경:** macOS (Apple Silicon 및 Intel)
- **현재 버전:** v0.5.0 (Binary Split Tree Layout)

### 핵심 철학 (Core Philosophy)

**VibeTerm**은 단순히 명령어를 입력하는 창을 넘어, **'바이브 코딩(Vibe Coding)'**에 최적화된 차세대 터미널 에뮬레이터입니다. Claude Code, Codex와 같은 AI CLI 도구를 사용하는 개발자가 터미널 환경을 떠나지 않고도 IDE 이상의 생산성을 경험하도록 설계되었습니다.

* **The Orchestrator:** 터미널은 단순한 텍스트 출력을 넘어, AI와 개발자가 공유하는 지능형 캔버스가 되어야 합니다.
* **Environment Control:** AI 도구가 실행되는 부모 환경(Terminal Emulator)을 지능화하여 컨텍스트를 주입하고 결과를 시각화합니다.

---

## 2. 기술 스택 (Tech Stack)

### 현재 스택

| 구성요소 | 라이브러리 | 버전 | 역할 |
|---------|-----------|------|------|
| 언어 | Rust | Stable | 메모리 안전 시스템 프로그래밍 |
| GUI Framework | egui + eframe | 0.31 | Immediate Mode GUI |
| 터미널 위젯 | egui_term | 0.1 | Alacritty 백엔드 기반 터미널 |
| 네이티브 메뉴 | muda | 0.15 | macOS 네이티브 메뉴바 |
| 설정 | serde + toml | 1.0 / 0.8 | 설정 파일 직렬화 |
| 비동기 | tokio | 1.0 | PTY 비동기 처리 |
| 로깅 | log + env_logger | 0.4 / 0.11 | 디버깅 로그 |

### 추가 예정

| 구성요소 | 라이브러리 | 역할 |
|---------|-----------|------|
| 파일 감시 | notify | 파일 시스템 변경 실시간 감지 |
| MCP 통신 | (TBD) | Model Context Protocol 지원 |

### 이전 시도 (폐기됨)
- ~~Iced 기반~~: 한글 입력/렌더링 성능 이슈
- ~~ratatui TUI 기반~~: iTerm2 등 호스트 터미널과 키보드 충돌
- ~~wgpu 직접 렌더링~~: 복잡도 높음, egui_term으로 대체

---

## 3. 현재 아키텍처

### 레이아웃 구조 (v0.5.0 - Binary Split Tree)
```
+------------------------------------------+
| [macOS Native Menu Bar]                   |   <- System Menu
+------------------------------------------+
|  [Tab Bar]    ▶1 shell │ 2 file.rs   +   |   <- 24px (TUI style)
+--------+---------------------------------+
| [Side  |   [Terminal Area]               |
|  bar]  |   +-------------+-------------+ |
|        |   |  Pane 1    ║  Pane 2     | |
| Files  |   |  (focused) ║             | |
|        |   +-----+------+-------------+ |
|  Tree  |         ║  Pane 3            | |
|        |         ║  (nested split)    | |
+--------+---------------------------------+
|  [Status Bar]  VibeTerm │ Panes: 3 │ ... |   <- 18px
+------------------------------------------+

Drag & Drop: 패널을 드래그하여 Top/Bottom/Left/Right 영역에 드롭
```

### 상태 관리 구조 (v0.5.0)
```
VibeTermApp
├── config: Config                    // 사용자 설정
│   ├── theme: ThemeConfig           // 색상 테마
│   ├── font: FontConfig             // 폰트 설정
│   └── ui: UiConfig                 // UI 레이아웃
├── theme: RuntimeTheme              // 파싱된 Color32 값
├── cached_terminal_theme            // 캐시된 터미널 테마 (성능 최적화)
├── workspaces: Vec<Workspace>       // 탭별 워크스페이스
│   └── Workspace
│       ├── name: String             // 탭 이름
│       ├── root: LayoutNode<TabContent>  // Binary Split Tree (NEW)
│       ├── focused_pane: PaneId     // 포커스된 패널 ID
│       └── next_pane_id: u64        // 다음 패널 ID
├── active_workspace: usize          // 활성 탭
├── sidebar_entries: Vec<FileEntry>  // 파일 트리
├── sidebar_selected: Option<usize>  // 선택된 파일
├── show_preferences: bool           // 설정 창 표시
├── ime_composing: bool              // IME 조합 중 여부
├── dragging_divider: Option<...>    // 디바이더 드래그 상태
└── dragging_pane: Option<PaneDragState>  // 패널 드래그 상태 (NEW)
```

### Binary Split Tree 구조 (src/layout.rs)
```rust
pub enum LayoutNode<T> {
    Leaf { id: PaneId, content: T },
    Split {
        direction: SplitDirection,  // Horizontal | Vertical
        ratio: f32,                 // 0.0-1.0
        first: Box<LayoutNode<T>>,
        second: Box<LayoutNode<T>>,
    },
}
```

---

## 4. 구현 상태

### ✅ 완료된 기능 (v0.5.0)

#### 핵심 터미널 기능
- [x] **egui_term 기반 터미널 렌더링**
  - Alacritty 백엔드 사용
  - PTY 프로세스 생성 및 통신
  - ANSI 이스케이프 시퀀스 완벽 지원
- [x] **Binary Split Tree 레이아웃** (NEW)
  - 수평 분할 (Cmd+D) - left | right
  - 수직 분할 (Cmd+Shift+D) - top / bottom
  - 무한 중첩 분할 지원
  - 클릭으로 패널 포커스 전환
  - 디바이더 드래그로 크기 조절
  - Ctrl+Tab/Ctrl+Shift+Tab 패널 순환
- [x] **드래그 앤 드롭 패널 재배치** (NEW)
  - iTerm2 스타일 패널 재배치
  - 8px 임계값 후 드래그 활성화
  - Top/Bottom/Left/Right 드롭 존 하이라이트
  - ESC로 드래그 취소
  - 트리 구조 extract/insert 함수
- [x] **성능 최적화** (NEW)
  - O(n²) → O(n) 렌더 루프 최적화
  - 조건부 레이아웃 재계산
  - 터미널 테마 캐싱
  - 배치 입력 상태 읽기
  - 20fps 유휴 repaint (CPU 절약)
- [x] **탭 시스템**
  - 새 탭 생성 (Cmd+T, + 버튼)
  - 탭 전환 (클릭, Cmd+1-9)
  - 탭 닫기 (Cmd+W, 중간 클릭)

#### UI/UX
- [x] **macOS 네이티브 메뉴바** (muda 크레이트)
- [x] **TUI 스타일 UI** (Box-drawing 문자)
- [x] **사이드바 파일 탐색기**
- [x] **설정 창** (Cmd+,)
- [x] **TOML 기반 테마 커스터마이징**

#### 폰트 및 국제화
- [x] **CJK 폰트 지원** (한글/일본어/중국어)
- [x] **IME 지원** (ViewportCommand::IMEAllowed)

---

## 5. Vibe Coding 기능 로드맵

### Phase 1: 기반 기능 완성 (v0.6.0) - **HIGH PRIORITY**

#### 🔥 Multi-Pane Contextual Sidebar (Dynamic File Tree) - **NEXT**
터미널 패널 전환 시 사이드바 파일 트리가 자동으로 해당 작업 디렉토리를 표시

| 요구사항 | 상세 |
|---------|------|
| **Core Behavior** | 패널 포커스 변경 → 사이드바 루트 자동 전환 |
| **Sidebar Header** | 열린 터미널 패널 목록 (아이콘/탭 형태) |
| **Smart Root Sync** | cd 명령 시 프로젝트 루트(.git, Cargo.toml) 자동 탐지 |
| **PTY Sync** | CWD 변경 감지 (portable-pty 이벤트 감시) |
| **Performance** | 비동기 디렉토리 로드 (UI 프리징 방지) |

**구현 가이드:**
```rust
// Pane 상태 확장
struct Pane {
    current_path: PathBuf,  // 현재 작업 디렉토리
    // ...
}

// 메시지 추가
enum Message {
    PaneFocused(PaneId),
    DirectoryChanged(PaneId, PathBuf),
    // ...
}
```

**User Story:**
1. 유저가 1번 패널(Backend)을 클릭 → 사이드바가 백엔드 프로젝트 폴더 표시
2. 유저가 2번 패널(Frontend)로 전환 → 사이드바가 프론트엔드 프로젝트 폴더 표시
3. 맥락 전환(Context Switching) 비용 최소화

---

#### 터미널 기본 기능
- [ ] 스크롤백 버퍼
- [ ] 텍스트 선택 및 복사
- [ ] 탭 드래그 앤 드롭 재정렬
- [ ] 새 창 열기 (Cmd+Shift+N)
- [ ] Command Palette (Cmd+P)

#### UI 개선
- [ ] 배경 투명도 (Opacity) 설정
- [ ] 배경 흐림 효과 (Blur)
- [ ] Active Line Highlight
- [ ] 커서 스타일 설정 (Block, Bar, Underline)

---

### Phase 2: 지능형 컨텍스트 관리 (v0.7.0)

#### 🧠 Smart Context Management
- [ ] **Auto-Context Pinning**
  - 현재 작업 중인 파일 및 디렉토리 구조를 AI 세션에 자동 태깅
  - AI가 현재 맥락을 즉시 파악
- [ ] **PTY Interception**
  - PTY 스트림 실시간 파싱
  - 에러 로그 및 특정 패턴 감지
  - 감지된 이벤트를 UI에 반영
- [ ] **Semantic Search**
  - `@` 키워드로 프로젝트 내 심볼(함수, 클래스) 검색
  - 프롬프트에 즉시 포함

#### Smart Sidebar 업그레이드
- [ ] **Pinned Files 표시**
  - AI가 현재 참고 중인 파일 하이라이트
- [ ] **Git Status 통합**
  - 변경된 파일 시각적 표시
- [ ] **File Watcher (`notify` 크레이트)**
  - 파일 시스템 변경 실시간 감지
  - AI 작업 결과 UI 자동 업데이트

---

### Phase 3: 실시간 코드 적용 (v0.8.0)

#### ⚡ Actionable Output
- [ ] **Ghost Text & Diff Preview**
  - AI 제안 변경사항을 반투명 오버레이로 시각화
  - 별도 패널에서 상세 Diff 표시
- [ ] **One-tap Apply**
  - `y` 키로 제안 코드 즉시 적용
  - git commit 연동 옵션

#### AI Inspector 패널 (Right Panel)
- [ ] **Thought Trace**
  - AI의 추론 과정 실시간 표시
- [ ] **상세 Diff View**
- [ ] **토큰 사용량 모니터**
- [ ] **실시간 비용 리포트**

---

### Phase 4: 멀티 세션 오케스트레이션 (v0.9.0)

#### 🤝 Multi-Session / MCP
- [ ] **Claude Event Bus (MCP)**
  - Model Context Protocol 활용
  - 여러 터미널 세션 간 상태/이벤트 공유
- [ ] **Global Context Dashboard**
  - 여러 패널의 AI 작업 상태 통합 모니터링
- [ ] **Smart Handoff**
  - 세션 A의 맥락을 세션 B로 즉시 전송
  - 다른 모델/작업 영역 간 컨텍스트 이전

---

### Phase 5: Vibe UI 완성 (v1.0.0)

#### 🎨 The "Vibe" UI
- [ ] **Aura Effect**
  - AI 작업 중 터미널 테두리에 그라데이션 애니메이션
- [ ] **Smooth Animations**
  - 패널 전환, 탭 전환 애니메이션
- [ ] **Customizable Themes**
  - AI 전용 색상 (Aura, Highlight, Selection)

---

## 6. 우선순위 정렬 (Priority Matrix)

| 우선순위 | 기능 | 이유 | 예상 복잡도 |
|---------|------|------|------------|
| **P0** | Multi-Pane Contextual Sidebar | 멀티 프로젝트 워크플로우 핵심 | Medium |
| **P1** | 스크롤백 버퍼 | 터미널 기본 기능 | Low |
| **P1** | 텍스트 선택/복사 | 터미널 기본 기능 | Medium |
| **P2** | Command Palette | 파워 유저 생산성 | Medium |
| **P2** | 탭 드래그 재정렬 | UX 개선 | Low |
| **P3** | 배경 투명도/블러 | 미관 | Low |
| **P3** | Git Status 통합 | 컨텍스트 향상 | Medium |

---

## 7. 설정 명세 (Preferences Specification)

### 현재 구현된 설정

```toml
[theme]
background = "#2E1A16"
surface = "#3A241E"
primary = "#E07A5F"
text = "#F4F1DE"
text_dim = "#A0968A"
border = "#4A2E28"

[font]
family = "JetBrains Mono"
size = 13.0
```

### 목표 설정 스펙

#### ⚙️ 일반 및 외관 (General & Appearance)

```toml
[general]
default_shell = "/bin/zsh"
scrollback_lines = 10000
initial_directory = "~"

[font]
family = "JetBrains Mono"
size = 13.0
line_height = 1.2
ligatures = true

[window]
opacity = 1.0          # 0.0 - 1.0
blur = false
padding = 8

[cursor]
style = "block"        # block, bar, underline
blink_speed = 500      # ms, 0 to disable
```

#### 🎨 색상 테마 (Color Schemes)

```toml
[theme]
# Base colors
background = "#2E1A16"
surface = "#3A241E"
primary = "#E07A5F"
text = "#F4F1DE"
text_dim = "#A0968A"
border = "#4A2E28"

# ANSI 16 colors
black = "#2E1A16"
red = "#E07A5F"
green = "#81B29A"
yellow = "#F2CC8F"
blue = "#3D405B"
magenta = "#B5838D"
cyan = "#6D9DC5"
white = "#F4F1DE"

# Vibe special (AI 전용)
aura_color = "#E07A5F"
ai_text_highlight = "#81B29A"
selection_color = "#3A241E"
ghost_text_color = "#A0968A"
```

#### 🤖 AI/연동 설정 (AI Specifics)

```toml
[ai]
enabled = true
mcp_bus_path = "/tmp/vibeterm-mcp.sock"
mcp_servers = ["claude", "codex"]

[ai.cost]
token_budget_limit = 100000     # daily limit
warning_threshold = 0.8         # 80% 경고

[ai.context]
ignored_patterns = ["node_modules", ".git", "target"]
watcher_debounce_ms = 100
auto_pin_opened_files = true
```

---

## 8. 키보드 단축키

### 현재 구현됨

| 단축키 | 기능 |
|--------|------|
| `Cmd+T` | 새 탭 |
| `Cmd+W` | 현재 패널/탭 닫기 |
| `Cmd+D` | 수평 분할 |
| `Cmd+Shift+D` | 수직 분할 ✅ NEW |
| `Cmd+B` | 사이드바 토글 |
| `Cmd+,` | 설정 창 |
| `Cmd+1-9` | 탭 전환 |
| `Ctrl+Tab` | 다음 패널로 이동 |
| `Ctrl+Shift+Tab` | 이전 패널로 이동 |
| `ESC` | 드래그 취소 |

### 추가 예정

| 단축키 | 기능 |
|--------|------|
| `Cmd+Shift+N` | 새 창 |
| `Cmd+P` | Command Palette |
| `Cmd+Shift+P` | AI Command Palette |
| `@` | Semantic Search (터미널 내) |
| `y` | AI 제안 적용 |

---

## 9. 파일 구조

### 현재 구조 (v0.5.0)
```
src/
├── main.rs          # 진입점, eframe 실행
├── app.rs           # VibeTermApp (메인 애플리케이션 상태)
├── layout.rs        # Binary Split Tree 레이아웃 시스템 (NEW)
├── config.rs        # Config, ThemeConfig, RuntimeTheme
├── menu.rs          # 네이티브 메뉴바 (muda)
├── theme.rs         # 테마 적용, TUI 문자, CJK 폰트 로드
└── ui/
    ├── mod.rs       # UI 모듈 익스포트
    ├── tab_bar.rs   # 탭바 컴포넌트
    ├── sidebar.rs   # 사이드바 파일 탐색기
    └── status_bar.rs # 상태바 컴포넌트

~/.config/vibeterm/
└── config.toml      # 사용자 설정 파일
```

### 추가 예정 구조

```
src/
├── ai/
│   ├── mod.rs           # AI 모듈
│   ├── context.rs       # 컨텍스트 관리
│   ├── mcp.rs           # MCP 통신
│   └── inspector.rs     # AI Inspector 패널
├── watcher/
│   └── mod.rs           # 파일 감시 (notify)
└── ui/
    ├── command_palette.rs  # Cmd+P
    ├── ghost_text.rs       # AI 제안 오버레이
    └── ai_inspector.rs     # 우측 AI 패널
```

---

## 10. 빌드 및 실행

```bash
# 빌드
cargo build --release

# 실행
cargo run

# 로그 확인
RUST_LOG=info cargo run 2>&1 | tee vibeterm.log
```

---

## 11. 알려진 제한사항

### 한글 IME 이슈
- **현상**: 일부 환경에서 한글 입력 시 자음/모음이 분리
- **원인**: winit/egui의 네이티브 IME 지원 한계
  - [winit#1497](https://github.com/rust-windowing/winit/issues/1497)
  - [egui#248](https://github.com/emilk/egui/issues/248)

---

## 12. 핸드오프 노트

### 마지막 작업 상태 (2025-01-26)
- ✅ egui_term 마이그레이션 완료
- ✅ 네이티브 메뉴바 구현
- ✅ 패널 포커스 전환 (클릭)
- ✅ 디바이더 드래그 (크기 조절)
- ✅ 파일 더블클릭으로 새 탭 열기
- ✅ TOML 기반 테마 커스터마이징
- ✅ CJK 폰트 자동 로드
- ✅ 한글 IME 지원 (ViewportCommand::IMEAllowed)
- ✅ 디바이더 드래그 시 음수 크기 패닉 수정
- ✅ **Binary Split Tree 레이아웃 구현** (NEW)
- ✅ **수직 분할 Cmd+Shift+D** (NEW)
- ✅ **드래그 앤 드롭 패널 재배치** (NEW)
- ✅ **O(n²) → O(n) 렌더 루프 최적화** (NEW)
- ✅ **터미널 테마 캐싱** (NEW)
- ✅ **조건부 repaint (CPU 절약)** (NEW)

### 다음 우선순위 작업
1. **Multi-Pane Contextual Sidebar** - 패널 전환 시 사이드바 동적 전환
2. 스크롤백 버퍼
3. 텍스트 선택/복사
4. Command Palette (Cmd+P)
