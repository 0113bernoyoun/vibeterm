# VibeTerm 상세 스펙 문서 (PRD)

## 1. 프로젝트 개요
- **명칭:** VibeTerm (AI-Native Tiling Terminal)
- **개발 환경:** macOS (Apple Silicon 및 Intel)
- **핵심 목표:** 멀티 프로젝트/터미널 타일링 레이아웃 구현 및 가벼운 네이티브 퍼포먼스 확보
- **현재 버전:** v0.4.0 (egui_term 기반)

## 2. 기술 스택 (Tech Stack)

| 구성요소 | 라이브러리 | 버전 | 역할 |
|---------|-----------|------|------|
| 언어 | Rust | Stable | 메모리 안전 시스템 프로그래밍 |
| GUI Framework | egui + eframe | 0.31 | Immediate Mode GUI |
| 터미널 위젯 | egui_term | 0.1 | Alacritty 백엔드 기반 터미널 |
| 네이티브 메뉴 | muda | 0.15 | macOS 네이티브 메뉴바 |
| 설정 | serde + toml | 1.0 / 0.8 | 설정 파일 직렬화 |
| 비동기 | tokio | 1.0 | PTY 비동기 처리 |
| 로깅 | log + env_logger | 0.4 / 0.11 | 디버깅 로그 |

### 이전 시도 (폐기됨)
- ~~Iced 기반~~: 한글 입력/렌더링 성능 이슈
- ~~ratatui TUI 기반~~: iTerm2 등 호스트 터미널과 키보드 충돌
- ~~wgpu 직접 렌더링~~: 복잡도 높음, egui_term으로 대체

## 3. 현재 아키텍처

### 레이아웃 구조
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
|        |   +============+-------------+ |
|  Tree  |   Divider (draggable, 4px)     |
|        |                                 |
+--------+---------------------------------+
|  [Status Bar]  VibeTerm │ Panes: 2 │ ... |   <- 18px
+------------------------------------------+
```

### 상태 관리 구조
```
VibeTermApp
├── config: Config                    // 사용자 설정
│   ├── theme: ThemeConfig           // 색상 테마
│   ├── font: FontConfig             // 폰트 설정
│   └── ui: UiConfig                 // UI 레이아웃
├── theme: RuntimeTheme              // 파싱된 Color32 값
├── workspaces: Vec<Workspace>       // 탭별 워크스페이스
│   └── Workspace
│       ├── name: String             // 탭 이름
│       ├── panes: Vec<Pane>         // 분할 패널들
│       │   └── Pane
│       │       ├── content: TabContent  // Terminal 또는 FileViewer
│       │       └── width_ratio: f32     // 상대 너비
│       └── focused_pane: usize      // 포커스된 패널 인덱스
├── active_workspace: usize          // 활성 탭
├── sidebar_entries: Vec<FileEntry>  // 파일 트리
├── sidebar_selected: Option<usize>  // 선택된 파일
├── show_preferences: bool           // 설정 창 표시
├── ime_composing: bool              // IME 조합 중 여부
└── dragging_divider: Option<...>    // 디바이더 드래그 상태
```

### 렌더링 파이프라인
1. **egui Frame**: eframe이 OpenGL(glow) 컨텍스트 관리
2. **Panel Layout**: TopBottomPanel(탭바, 상태바), SidePanel(사이드바), CentralPanel(터미널)
3. **Terminal Rendering**: egui_term이 Alacritty 백엔드로 터미널 렌더링
4. **UI Widgets**: egui Button, Label 등으로 TUI 스타일 UI

### 테마 시스템 (Dark Brown - 사용자 정의 가능)
| 요소 | 기본 색상 | Hex |
|------|----------|-----|
| Background | 다크 브라운 | `#2E1A16` |
| Surface | 밝은 브라운 | `#3A241E` |
| Primary | 코랄 | `#E07A5F` |
| Text | 크림 | `#F4F1DE` |
| Text Dim | 회색 | `#A0968A` |
| Border | 브라운 | `#4A2E28` |

설정 파일: `~/.config/vibeterm/config.toml`

## 4. 구현 상태

### ✅ 완료된 기능

#### 핵심 터미널 기능
- [x] **egui_term 기반 터미널 렌더링**
  - Alacritty 백엔드 사용
  - PTY 프로세스 생성 및 통신
  - ANSI 이스케이프 시퀀스 완벽 지원
- [x] **멀티 패널 분할** (Cmd+D)
  - 수평 분할 지원
  - 클릭으로 패널 포커스 전환
  - 디바이더 드래그로 크기 조절
- [x] **탭 시스템**
  - 새 탭 생성 (Cmd+T, + 버튼)
  - 탭 전환 (클릭, Cmd+1-9)
  - 탭 닫기 (Cmd+W, 중간 클릭)

#### UI/UX
- [x] **macOS 네이티브 메뉴바** (muda 크레이트)
  - VibeTerm, File, Edit, View, Shell, Window, Help 메뉴
  - 메뉴 항목에 키보드 단축키 표시
- [x] **TUI 스타일 UI**
  - Box-drawing 문자 (─, │, ┌, ┐ 등)
  - 모노스페이스 폰트
  - ASCII 스타일 아이콘 ([+], [-], ├──)
- [x] **사이드바 파일 탐색기**
  - 디렉토리 펼치기/접기
  - 파일 선택 하이라이트
  - **더블클릭으로 파일을 새 탭으로 열기**
- [x] **설정 창** (Cmd+, 또는 메뉴)
  - 테마 색상 커스터마이징
  - TOML 파일 저장/로드

#### 폰트 및 국제화
- [x] **CJK 폰트 지원**
  - macOS: Apple SD Gothic Neo 자동 로드
  - Linux: Noto Sans CJK 자동 로드
  - 폴백 폰트로 한글/일본어/중국어 표시
- [x] **IME 지원 시도**
  - `ViewportCommand::IMEAllowed(true)` 활성화
  - IME 이벤트 핸들링 구현

### ⚠️ 알려진 제한사항

#### 한글 IME 이슈
- **현상**: 한글 입력 시 자음/모음이 분리되어 표시 (예: ㅎㅏㄴㄱㅡㄹ)
- **원인**: winit/egui의 네이티브 IME 지원 한계
  - [winit#1497](https://github.com/rust-windowing/winit/issues/1497)
  - [egui#248](https://github.com/emilk/egui/issues/248)
- **영향받는 터미널**: Alacritty도 동일 이슈 있음
- **대안**:
  1. WezTerm 기반으로 전환 (자체 UI 포기 필요)
  2. macOS Cocoa API 직접 사용 (전체 재작성 필요)
  3. 현재 상태 유지 (영문/숫자만 정상)

### 🚧 개발 예정
- [ ] 한글 IME 완전 지원 (아키텍처 결정 필요)
- [ ] 수직 분할 (Cmd+Shift+D)
- [ ] 스크롤백 버퍼
- [ ] 텍스트 선택 및 복사
- [ ] 탭 드래그 앤 드롭 재정렬
- [ ] 새 창 열기 (Cmd+Shift+N)

## 5. 키보드 단축키

| 단축키 | 기능 |
|--------|------|
| `Cmd+T` | 새 탭 |
| `Cmd+W` | 현재 패널/탭 닫기 |
| `Cmd+D` | 수평 분할 |
| `Cmd+B` | 사이드바 토글 |
| `Cmd+,` | 설정 창 |
| `Cmd+1-9` | 탭 전환 |
| `Ctrl+Tab` | 다음 패널로 이동 |
| `Ctrl+Shift+Tab` | 이전 패널로 이동 |

## 6. 파일 구조

```
src/
├── main.rs          # 진입점, eframe 실행
├── app.rs           # VibeTermApp (메인 애플리케이션 상태)
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

## 7. 빌드 및 실행

```bash
# 빌드
cargo build --release

# 실행
cargo run

# 로그 확인
RUST_LOG=info cargo run 2>&1 | tee vibeterm.log
```

## 8. 의존성 (Cargo.toml)

```toml
[dependencies]
egui = "0.31"
eframe = { version = "0.31", features = ["glow", "persistence"] }
egui_term = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "sync", "macros"] }
muda = "0.15"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
anyhow = "1"
log = "0.4"
env_logger = "0.11"
dirs = "5"
```

---

## 9. 핸드오프 노트

### 마지막 작업 상태 (2025-01-25)
- ✅ **egui_term 마이그레이션 완료**: wgpu에서 egui+egui_term으로 전환
- ✅ **네이티브 메뉴바 구현**: muda 크레이트로 macOS 메뉴바 추가
- ✅ **패널 포커스 전환**: 클릭으로 분할 패널 간 포커스 전환
- ✅ **디바이더 드래그**: 패널 크기 조절 가능
- ✅ **파일 더블클릭**: 사이드바에서 파일을 새 탭으로 열기
- ✅ **설정 시스템**: TOML 기반 테마 커스터마이징
- ✅ **CJK 폰트 로드**: 시스템 한글 폰트 자동 로드
- ⚠️ **한글 IME**: ViewportCommand::IMEAllowed 활성화했으나 winit 한계로 미완성

### 한글 IME 해결을 위한 옵션

| 옵션 | 장점 | 단점 |
|------|------|------|
| **WezTerm 기반** | IME 지원 좋음 | 커스텀 UI 포기 |
| **Cocoa API** | 완벽한 IME | 전체 재작성 |
| **현재 유지** | 안정적 | 영문만 정상 |

### 다음 우선순위 작업
1. **한글 IME 해결 방향 결정**
2. **수직 분할** (Cmd+Shift+D)
3. **스크롤백 버퍼**
4. **텍스트 선택/복사**
