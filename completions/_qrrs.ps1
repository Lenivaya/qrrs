
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'qrrs' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'qrrs'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'qrrs' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Format in which the qrcode will be saved')
            [CompletionResult]::new('--output-format', '--output-format', [CompletionResultType]::ParameterName, 'Format in which the qrcode will be saved')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Margin applied to qrcode')
            [CompletionResult]::new('--margin', '--margin', [CompletionResultType]::ParameterName, 'Margin applied to qrcode')
            [CompletionResult]::new('--generate-completions', '--generate-completions', [CompletionResultType]::ParameterName, 'Generate completion file for the specified shell')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Error correction level for the QR code')
            [CompletionResult]::new('--error-correction-level', '--error-correction-level', [CompletionResultType]::ParameterName, 'Error correction level for the QR code')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Read the qrcode instead of generating it')
            [CompletionResult]::new('--read', '--read', [CompletionResultType]::ParameterName, 'Read the qrcode instead of generating it')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Display code in terminal')
            [CompletionResult]::new('--terminal', '--terminal', [CompletionResultType]::ParameterName, 'Display code in terminal')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Invert qrcode colors')
            [CompletionResult]::new('--invert_colors', '--invert_colors', [CompletionResultType]::ParameterName, 'Invert qrcode colors')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
