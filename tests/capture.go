package tests

import (
	"log"
	"regexp"
)

var capturePattern = regexp.MustCompile(`{{([a-zA-Z0-9_]+)}}`)
var embedPattern = regexp.MustCompile(`\(\(([a-zA-Z0-9_]+)\)\)`)

func capture(template, target string, variables *map[string]string) string {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	// 変数をキャプチャするための正規表現を組み立てる
	variableDefinitionMatches := capturePattern.FindAllStringSubmatchIndex(template, -1)
	patternStr := ""
	last := 0
	var names []string
	for _, match := range variableDefinitionMatches {
		patternStr += regexp.QuoteMeta(template[last:match[0]])
		patternStr += "([a-zA-Z0-9-_=\\.]+)"
		last = match[1]
		names = append(names, template[match[2]:match[3]])
	}
	patternStr += regexp.QuoteMeta(template[last:])

	// 埋め込み用のパターン部分を避ける
	patternStr = regexp.MustCompile(`\\\(\\\([a-zA-Z0-9_]+\\\)\\\)`).ReplaceAllString(patternStr, "[a-zA-Z0-9-_=\\.]+")

	pattern, err := regexp.Compile(patternStr)
	if err != nil {
		panic(err)
	}

	// 変数のキャプチャ
	variableCaptureMatches := pattern.FindStringSubmatch(target)

	if len(variableCaptureMatches) >= 2 {
		for i, name := range names {
			(*variables)[name] = variableCaptureMatches[i+1]
		}
	}

	// 今回キャプチャした変数を埋め込んでいく
	replaced := ""
	last = 0
	for _, match := range variableDefinitionMatches {
		replaced += template[last:match[0]]
		v, ok := (*variables)[template[match[2]:match[3]]]
		if ok {
			replaced += v
		} else {
			replaced += template[match[2]:match[3]]
		}
		last = match[1]
	}
	replaced += template[last:]

	// ここから既存の変数を埋め込んでいく
	variableEmbedMatches := embedPattern.FindAllStringSubmatchIndex(replaced, -1)
	if len(variableEmbedMatches) == 0 {
		return replaced
	}

	embed := ""
	last = 0
	for _, match := range variableEmbedMatches {
		embed += replaced[last:match[0]]

		v, ok := (*variables)[replaced[match[2]:match[3]]]
		if ok {
			embed += v
		} else {
			embed += replaced[match[0]:match[1]]
		}

		last = match[1]
	}
	embed += replaced[last:]
	return embed
}
