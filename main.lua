FPS = 40
FRAME_DURATION = 1000 / FPS

moon = gfx.load_img("assets/moon.png")
x = -256
y = 0
local up
local down
local left
local right
while x < 800 and not inp.key_down('Escape') do
    local frame_start = ticks()

    up    = inp.key_down('W') or inp.key_down('Up')
    down  = inp.key_down('S') or inp.key_down('Down')
    left  = inp.key_down('A') or inp.key_down('Left')
    right = inp.key_down('D') or inp.key_down('Right')

    gfx.clear_screen()
    gfx.draw_img(moon, x, y)
    gfx.display()
    
    if left and not right then
        x = x - 1
    elseif right and not left then
        x = x + 1
    else
        x = x + 2
    end
    if up and not down then
        y = y - 1
    elseif down and not up then
        y = y + 1
    else
        y = math.sin(x / 100) * 200 + 300 - 128
    end

    sleep(FRAME_DURATION - ticks() + frame_start)
end