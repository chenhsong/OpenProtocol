using System;

namespace iChen.OpenProtocol
{
	[Flags]
	public enum Filters
	{
		None = 0x0,

		// Message filters

		Status = 0x00000001,
		Cycle = 0x00000002,
		Mold = 0x00000004,
		Actions = 0x00000008,
		Alarms = 0x00000010,
		Audit = 0x00000020,
		All = 0x000000ff,

		// MIS integration

		JobCards = 0x00001000,
		Operators = 0x00002000,

		OPCUA = 0x10000000
	}

	public enum Languages
	{
		Unknown = 0,
		EN, B5, GB, FR, DE, IT, ES, PT, JA
	}

	public enum ControllerTypes
	{
		Unknown = 99,
		Ai01 = 1,
		Ai11 = 2,
		Ai02 = 6,
		Ai12 = 7,
		CPC60 = 8,
		MPC60 = 9,
		CDC2000 = 98,
		CDC3000 = 4,
		CDC2000WIN = 3,
		SPS3300 = 0,
		NewAge = 5,
		CBmold300 = 10,
		CBmold800 = 11,
		MPC7 = 12
	}

	public enum OpModes
	{
		Unknown = 0,
		Manual,
		SemiAutomatic,
		Automatic,
		Others,
		Offline = 99
	}

	public enum JobModes
	{
		Unknown = 0,
		ID01, ID02, ID03, ID04, ID05, ID06, ID07, ID08, ID09, ID10, ID11, ID12, ID13, ID14, ID15,
		Offline = 99
	}
}