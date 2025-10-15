namespace wpc;

[System.Runtime.Versioning.SupportedOSPlatform(@"windows")]
internal static class CommandHandlers
{
	public static readonly Argument<string> UrlArgument = new(@"url");

	public static readonly Option<string> BypassOption = new(@"--bypass", @"-b") { DefaultValueFactory = _ => string.Join(@";", ProxyService.LanIp) };

	public static void Query(ParseResult parseResult)
	{
		using ProxyService service = new();
		ProxyStatus status = service.Query();
		Console.WriteLine($@"IsDirect:       {status.IsDirect}");
		Console.WriteLine($@"IsProxy:        {status.IsProxy}");
		Console.WriteLine($@"IsAutoProxyUrl: {status.IsAutoProxyUrl}");
		Console.WriteLine($@"IsAutoDetect:   {status.IsAutoDetect}");
		Console.WriteLine($@"ProxyServer:    {status.ProxyServer}");
		Console.WriteLine($@"ProxyBypass:    {status.ProxyBypass}");

		if (!string.IsNullOrWhiteSpace(status.AutoConfigUrl))
		{
			Console.WriteLine($@"AutoConfigUrl:  {status.AutoConfigUrl}");
		}
	}

	public static int Direct(ParseResult parseResult)
	{
		using ProxyService service = new();
		return ReturnResult(service.Direct());
	}

	public static int Pac(ParseResult parseResult)
	{
		string url = parseResult.GetRequiredValue(UrlArgument);
		using ProxyService service = new() { AutoConfigUrl = url };
		return ReturnResult(service.Pac());
	}

	public static int Global(ParseResult parseResult)
	{
		string url = parseResult.GetRequiredValue(UrlArgument);
		string bypass = parseResult.GetRequiredValue(BypassOption);
		using ProxyService service = new()
		{
			Server = url,
			Bypass = bypass
		};
		return ReturnResult(service.Global());
	}

	private static int ReturnResult(bool result)
	{
		Console.WriteLine(result ? @"success" : @"fail");
		return Convert.ToInt32(!result);
	}
}
