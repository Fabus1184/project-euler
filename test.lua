#!/usr/bin/env lua

local os = require "os"
local io = require "io"
local string = require "string"
local stat = require "posix.sys.stat"
local time = require "posix.sys.time"

TMP_DIR = "build"
RESULT_FILE = "TEST_RESULT.md"

OUTPUT = false

function Millis()
    local t = time.gettimeofday()
    return (t.tv_sec * 1000) + (t.tv_usec / 1000)
end

function Run(file)
    local _, _, code = os.execute(string.format("ghc -O2 -o %s/%s -no-keep-hi-files -no-keep-o-files %s.hs > /dev/null",
        TMP_DIR
        , file, file))
    if (code ~= 0) then
        print(string.format("Compile %s.hs failed: %d", file, code))
        os.exit(code)
    end

    local t1 = Millis();
    _, _, code = os.execute(string.format("./%s/%s" ..
        (function() if OUTPUT then return "> /dev/null" else return "" end end)(), TMP_DIR, file))
    local t2 = Millis();
    if (code ~= 0) then
        print(string.format("Run %s failed: %d", file, code))
        os.exit(code)
    end

    Log(string.format("| %s | %s |", file, ColorTime(t2 - t1)))
end

function Log(msg)
    if (OUTPUT) then
        RESULT_FILE:write(msg .. "\n"):flush()
    end
end

function ColorTime(elapsed)
    if (elapsed < 50) then
        return string.format("<font color=\"green\">%.0fms</font>", elapsed)
    elseif (elapsed < 1000) then
        return string.format("<font color=\"orange\">%.0fms</font>", elapsed)
    else
        return string.format("<font color=\"red\">%.0fms</font>", elapsed)
    end
end

if arg[1] then
    OUTPUT = false
    Run(arg[1])
else
    OUTPUT = true

    io.open(RESULT_FILE, "w"):write("# Test Results\n\n"):close()
    RESULT_FILE = io.open(RESULT_FILE, "a")
    Log("| Problem | Time |")
    Log("| --- | ------- |")

    for i = 1, 500 do
        if stat.lstat(string.format("%d.hs", i)) then
            Run(tostring(i))
        else
            Log(string.format("| %d | **-** |", i))
        end
    end

    RESULT_FILE:close()
end
