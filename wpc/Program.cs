global using System.CommandLine;
global using System.CommandLine.Invocation;
global using System.CommandLine.NamingConventionBinder;
global using WindowsProxy;
global using wpc;

if (!OperatingSystem.IsWindows())
{
	Console.WriteLine(@"Only support windows now.");
	return 1;
}

Argument urlArgument = new(@"url");

string defaultBypass = string.Join(@";", ProxyService.LanIp);
Option<string> bypassOption = new(@"--bypass", () => defaultBypass);
bypassOption.AddAlias(@"-b");

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

queryCommand.Handler = CommandHandlers.Query();
directCommand.Handler = CommandHandlers.Direct();
pacCommand.Handler = CommandHandlers.Pac();
globalCommand.Handler = CommandHandlers.Global();

return root.Invoke(args);
