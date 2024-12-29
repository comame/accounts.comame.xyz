package tests

import (
	"errors"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

type scenario struct {
	Name string

	Steps []interface{}
}

type stepType string

const (
	stepTypeHttpRequest stepType = "httpRequest"
	stepTypeSQL         stepType = "sql"
)

type httpRequestStep struct {
	Type            stepType
	StepDescription string

	ReqMethod  string
	ReqPath    string
	ReqHeaders map[string]string
	ReqBody    string

	ResStatus  int
	ResHeaders map[string]string
	ResBody    string
}

type sqlStep struct {
	Type            stepType
	StepDescription string

	Query string
}

func GetScenarios() ([]scenario, error) {
	files, err := listFiles()
	if err != nil {
		return nil, err
	}

	var r []scenario
	for _, file := range files {
		s, err := func() (*scenario, error) {
			file := file
			f, err := os.Open("tests/scenarios/" + file)
			if err != nil {
				return nil, err
			}
			defer f.Close()

			fs, err := io.ReadAll(f)
			if err != nil {
				return nil, err
			}

			s, err := parseScenario(string(fs), file)
			if err != nil {
				return nil, err
			}

			return s, nil
		}()
		if err != nil {
			return nil, err
		}
		r = append(r, *s)
	}

	return r, nil
}

func listFiles() ([]string, error) {
	var r []string

	dirs, err := os.ReadDir("tests/scenarios")
	if err != nil {
		return nil, err
	}
	for _, dir := range dirs {
		files, err := os.ReadDir("tests/scenarios/" + dir.Name())
		if err != nil {
			return nil, err
		}
		for _, file := range files {
			r = append(r, dir.Name()+"/"+file.Name())
		}
	}

	return r, nil
}

func parseScenario(t string, name string) (*scenario, error) {
	var s scenario
	s.Name = name

	var steps []interface{}

	stepStrs := strings.Split(t, "\n\n\n\n\n") // 空行4つ
	for _, stepStr := range stepStrs {
		sp := strings.SplitN(stepStr, "\n", 2)
		if !(len(sp) == 1 || len(sp) == 2) {
			return nil, errors.New("stepヘッダ行が変")
		}

		switch {
		case strings.HasPrefix(sp[0], string(stepTypeHttpRequest)):
			step, err := parseHttpRequestStep(sp[1])
			if err != nil {
				return nil, err
			}
			step.StepDescription, _ = strings.CutPrefix(sp[0], string(stepTypeHttpRequest))
			steps = append(steps, *step)
		case strings.HasPrefix(sp[0], string(stepTypeSQL)):
			step, err := parseSQLStep(sp[1])
			if err != nil {
				return nil, err
			}
			step.StepDescription, _ = strings.CutPrefix(sp[0], string(stepTypeSQL))
			steps = append(steps, *step)
		default:
			return nil, fmt.Errorf("未知のstepType %s", sp[0])
		}
	}

	s.Steps = steps
	return &s, nil
}

func parseHttpRequestStep(t string) (*httpRequestStep, error) {
	var s httpRequestStep

	s.Type = stepTypeHttpRequest

	sp := strings.Split(t, "\n\n\n") // 空行2つ
	if len(sp) != 2 {
		return nil, errors.New("リクエストとレスポンス以外の何かが含まれてるorどっちがない")
	}

	srq, err := parseHttpRequestSection(sp[0])
	if err != nil {
		return nil, err
	}
	s.ReqMethod = srq.ReqMethod
	s.ReqPath = srq.ReqPath
	s.ReqHeaders = srq.ReqHeaders
	s.ReqBody = srq.ReqBody

	srs, err := parseHttpResponseSection(sp[1])
	if err != nil {
		return nil, err
	}
	s.ResStatus = srs.ResStatus
	s.ResHeaders = srs.ResHeaders
	s.ResBody = srs.ResBody

	return &s, nil
}

func parseHttpRequestSection(t string) (*httpRequestStep, error) {
	var s httpRequestStep

	lines := strings.Split(t, "\n")
	if len(lines) == 0 {
		return nil, errors.New("リクエストの1行目がない")
	}

	reqLine := strings.Split(lines[0], " ")
	if len(reqLine) != 2 {
		return nil, errors.New("リクエスト行の形式が変")
	}
	s.ReqMethod = reqLine[0]
	s.ReqPath = reqLine[1]
	if len(lines) == 1 {
		return &s, nil
	}
	lines = lines[1:]

	headers := make(map[string]string)
	lastHeaderLine := 0
	for _, l := range lines {
		if l == "" {
			break
		}

		if err := parseHeaderLine(&headers, l); err != nil {
			return nil, err
		}
		lastHeaderLine += 1
	}
	s.ReqHeaders = headers
	if len(lines) <= lastHeaderLine {
		return &s, nil
	}
	lines = lines[lastHeaderLine+1:]

	s.ReqBody = strings.Join(lines, "\n")
	return &s, nil
}

func parseHttpResponseSection(t string) (*httpRequestStep, error) {
	var s httpRequestStep

	lines := strings.Split(t, "\n")
	if len(lines) == 0 {
		return nil, errors.New("レスポンスの1行目がない")
	}

	status, err := strconv.Atoi(lines[0])
	if err != nil {
		return nil, errors.New("ステータスコードが数値ではない")
	}
	s.ResStatus = status
	if len(lines) == 1 {
		return &s, nil
	}
	lines = lines[1:]

	headers := make(map[string]string)
	lastHeaderLine := 0
	for _, l := range lines {
		if l == "" {
			break
		}

		if err := parseHeaderLine(&headers, l); err != nil {
			return nil, err
		}
		lastHeaderLine += 1
	}
	s.ResHeaders = headers
	if len(lines) <= lastHeaderLine {
		return &s, nil
	}
	lines = lines[lastHeaderLine+1:]

	s.ResBody = strings.Join(lines, "\n")
	return &s, nil
}

func parseHeaderLine(mp *map[string]string, line string) error {
	sp := strings.SplitN(line, ": ", 2)
	if len(sp) != 2 {
		return fmt.Errorf("ヘッダの形式が変 %s", line)
	}
	(*mp)[sp[0]] = sp[1]

	return nil
}

func parseSQLStep(t string) (*sqlStep, error) {
	var s sqlStep

	q := strings.TrimSpace(t)
	if len(q) == 0 {
		return nil, errors.New("空のクエリ")
	}

	s.Query = t
	s.Type = stepTypeSQL
	return &s, nil
}
