import math

start = -20
end = 19
spacing = 1.0
voxels = []

for z in range(start, end + 1):
    for y in range(start, end + 1):
        for x in range(start, end + 1):
            px = x * spacing
            py = y * spacing
            pz = z * spacing

            head = (px / 12.8) ** 2 + (py / 14.5) ** 2 + (pz / 10.3) ** 2 <= 1.0
            if not head:
                continue

            if py < -12 and abs(px) > 6 and abs(pz) < 5:
                continue
            if py < -10 and abs(px) < 9 and abs(pz) > 6:
                continue
            if pz < -10 and abs(py) < 4 and abs(px) > 8:
                continue

            cheek_scale = 1.0 - 0.18 * math.exp(-((py - 2.0) / 4.0) ** 2)
            if pz > 2.5 and abs(px) > 10.2 * cheek_scale and abs(py) < 4.5:
                continue

            temple = abs(px) > 9.0 and abs(pz) < 3.0 and y in range(0, 6)
            ear = abs(px) > 9.5 and abs(pz + 2.0) < 3.5 and abs(py - 1.5) < 4.5

            left_eye = x in (-5, -4, -3) and y in range(2, 5) and z == 9
            right_eye = x in (3, 4, 5) and y in range(2, 5) and z == 9
            left_socket = x in (-6, -5, -4, -3) and y in range(1, 5) and z in (8, 9)
            right_socket = x in (3, 4, 5, 6) and y in range(1, 5) and z in (8, 9)
            mouth = y == -4 and z in (8, 9, 10) and abs(x) <= 4
            upper_lip = y == -3 and z in (9, 10) and abs(x) <= 3
            nose = z in (8, 9, 10, 11) and y in (0, 1, 2, 3) and abs(x) <= 2
            brow = y in (4, 5, 6) and z in (7, 8, 9) and abs(x) <= 6
            hair = y >= 8 and pz >= 0 and abs(px) <= 11 and head

            brain = (px / 6.5) ** 2 + ((py - 1.5) / 6.2) ** 2 + (pz / 4.2) ** 2 <= 1.0
            brain_dist = math.sqrt((px / 6.5) ** 2 + ((py - 1.5) / 6.2) ** 2 + (pz / 4.2) ** 2)
            surface = head and math.sqrt((px / 12.8) ** 2 + (py / 14.5) ** 2 + (pz / 10.3) ** 2) > 0.82

            if left_eye or right_eye:
                color = (20, 20, 20)
            elif mouth:
                color = (190, 55, 55)
            elif upper_lip:
                color = (220, 80, 70)
            elif nose:
                color = (240, 190, 150)
            elif left_socket or right_socket:
                color = (80, 60, 55)
            elif hair:
                color = (40, 26, 15)
            elif temple:
                color = (220, 180, 150)
            elif brain:
                gray = int(240 - 70 * brain_dist)
                gray = max(110, min(240, gray))
                color = (gray, gray, gray)
            elif surface:
                r = 242
                g = 205
                b = 170
                if y > 7:
                    r, g, b = 75, 45, 35
                elif z > 4:
                    r, g, b = 225, 160, 130
                elif y < -6:
                    r, g, b = 205, 145, 120
                color = (r, g, b)
            else:
                color = (200, 105, 85)

            if ear:
                color = (230, 180, 150)
            if brow and not left_eye and not right_eye:
                color = (210, 140, 120)

            voxels.append((x, y, z, *color))

print(len(voxels))
with open('src/head_voxels.txt', 'w', encoding='utf-8') as f:
    f.write('# x y z r g b\n')
    for v in voxels:
        f.write(' '.join(str(i) for i in v) + '\n')
