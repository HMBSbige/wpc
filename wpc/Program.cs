global using System.CommandLine;
global using WindowsProxy;
global using wpc;

if (!OperatingSystem.IsWindows())
{
	Console.WriteLine(@"Only support windows now.");
	return 1;
}

Argument<string> urlArgument = new(@"url");

string defaultBypass = string.Join(@";", ProxyService.LanIp);
Option<string> bypassOption = new(@"--bypass")
{
	DefaultValueFactory = _ => defaultBypass
};
bypassOption.Aliases.Add(@"-b");

Command queryCommand = new(Constants.QueryCommand);
Command directCommand = new(Constants.DirectCommand);
Command pacCommand = new(Constants.PacCommand) { urlArgument };
Command globalCommand = new(Constants.GlobalCommand) { urlArgument, bypassOption };

RootCommand root = new()
{
	queryCommand,
	directCommand,
	pacCommand,
	globalCommand
};

queryCommand.SetAction(CommandHandlers.Query);
directCommand.SetAction(CommandHandlers.Direct);
pacCommand.SetAction(CommandHandlers.Pac);
globalCommand.SetAction(CommandHandlers.Global);

return root.Parse(args).Invoke();
