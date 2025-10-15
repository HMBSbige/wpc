namespace wpc;

[System.Runtime.Versioning.SupportedOSPlatform(@"windows")]
internal static class CommandHandlers
{
	public static void Query(ParseResult parseResult)
	{
		using ProxyService service = new();
		ProxyStatus status = service.Query();
		Console.WriteLine($@"IsDirect: {status.IsDirect}");
		Console.WriteLine($@"IsProxy: {status.IsProxy}");
		Console.WriteLine($@"IsAutoProxyUrl: {status.IsAutoProxyUrl}");
		Console.WriteLine($@"IsAutoDetect: {status.IsAutoDetect}");
		Console.WriteLine($@"ProxyServer: {status.ProxyServer}");
		Console.WriteLine($@"ProxyBypass: {status.ProxyBypass}");
		Console.WriteLine($@"AutoConfigUrl: {status.AutoConfigUrl}");
	}

	public static int Direct(ParseResult parseResult)
	{
		using ProxyService service = new();
		return ReturnResult(service.Direct());
	}

	public static int Pac(ParseResult parseResult)
	{
		string url = parseResult.GetValue<string>("url")!;
		using ProxyService service = new()
		{
			AutoConfigUrl = url
		};
		return ReturnResult(service.Pac());
	}

	public static int Global(ParseResult parseResult)
	{
		string url = parseResult.GetValue<string>("url")!;
		string bypass = parseResult.GetValue<string>("bypass")!;
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
