include("menu/panopticon_loading_policy.lua")

do
    local drawBackground, changeBackground = DrawBackground, ChangeBackground
    local clearImages, addImage = ClearBackgroundImages, AddBackgroundImage
    local picturesEnabled = false

    -- Штатное меню сканирует свои каталоги. Для этой сессии список задаёт Hub.
    function ClearBackgroundImages() end
    function AddBackgroundImage() end

    function ChangeBackground(...)
        if picturesEnabled then changeBackground(...) end
    end

    function DrawBackground()
        if IsInGame() and not IsInLoading() then
            drawBackground()
            return
        end

        surface.SetDrawColor(0, 0, 0, 255)
        surface.DrawRect(0, 0, ScrW(), ScrH())
        if picturesEnabled and not IsInLoading() then drawBackground() end
    end

    function PanopticonEnableBackgrounds()
        if picturesEnabled then return end
        picturesEnabled = true
        clearImages()
        addImage("../html/panopticon/backdrops/academy-01.jpg")
        addImage("../html/panopticon/backdrops/academy-02.jpg")
        addImage("../html/panopticon/backdrops/academy-03.jpg")
        changeBackground()
    end
end

do
    local getLoadPanel = GetLoadPanel
    local bridgedPanel

    local function publishLoading(loading)
        if not IsValid(pnlMainMenu) then return end
        pnlMainMenu:Call("if (window.SetInGame) window.SetInGame(" .. tostring(IsInGame()) ..
            "); if (window.SetLoading) window.SetLoading(" .. tostring(loading) .. ")")
    end

    if type(getLoadPanel) == "function" then
        function GetLoadPanel()
            local panel = getLoadPanel()
            if not IsValid(panel) or panel == bridgedPanel then return panel end
            local checkStatus = panel.CheckForStatusChanges
            if type(checkStatus) ~= "function" then return panel end
            bridgedPanel = panel

            local activate, deactivate, think = panel.OnActivate, panel.OnDeactivate, panel.Think
            local hideIndicator, escapeDown
            function panel:OnActivate()
                activate(self)
                if IsValid(self.HTML) then publishLoading(true) end
                local native = PanopticonNative
                hideIndicator = native and native.GetLoadingIndicator and native.HideLoadingDialog
                    and native.GetLoadingIndicator() == false
                escapeDown = input.IsKeyDown(KEY_ESCAPE)
            end

            function panel:OnDeactivate()
                deactivate(self)
                publishLoading(false)
            end

            function panel:Think()
                think(self)
                if not hideIndicator or not IsInLoading() then return end
                local pressed = input.IsKeyDown(KEY_ESCAPE)
                if PanopticonNative.HideLoadingDialog() and pressed and not escapeDown then
                    CancelLoading()
                end
                escapeDown = pressed
            end

            local lastHTML, lastStatus
            function panel:CheckForStatusChanges()
                local html = self.HTML
                if self.LoadedURL ~= "asset://garrysmod/html/loading.html" or not IsValid(html) then
                    lastHTML, lastStatus = nil, nil
                    return checkStatus(self)
                end
                if html:IsLoading() then return end

                -- The stock Lua callback strips the engine's FastDL/ServerDL queue position.
                local status = GetLoadStatus() or ""
                if html == lastHTML and status == lastStatus then return end
                lastHTML, lastStatus = html, status
                self:RunJavascript("if (window.STLoadingStatus) window.STLoadingStatus('" .. status:JavascriptSafe() .. "')")
            end

            return panel
        end
    end
end
