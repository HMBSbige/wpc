global using System.CommandLine;
global using WindowsProxy;
global using wpc;

if (!OperatingSystem.IsWindows())
{
	Console.WriteLine(@"Only support windows now.");
	return 1;
}

Command queryCommand = new(Constants.QueryCommand);
Command directCommand = new(Constants.DirectCommand);
Command pacCommand = new(Constants.PacCommand) { CommandHandlers.UrlArgument };
Command globalCommand = new(Constants.GlobalCommand)
{
	CommandHandlers.UrlArgument,
	CommandHandlers.BypassOption
};

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
